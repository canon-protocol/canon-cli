use crate::utils::{CanonError, CanonResult};
use canon_protocol::{CanonSpecification, Dependency, FieldType, SchemaField};
use console::style;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn run_validate(
    path: Option<String>,
    strict: bool,
    _schema: Option<String>,
    _fix: bool,
) -> CanonResult<()> {
    // Determine which file to validate
    let canon_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        std::env::current_dir()
            .map_err(|e| CanonError::Command {
                message: format!("Failed to get current directory: {}", e),
            })?
            .join("canon.yml")
    };

    if !canon_path.exists() {
        return Err(CanonError::Command {
            message: format!("File not found: {}", canon_path.display()),
        });
    }

    println!(
        "{} {}",
        style("Validating").cyan().bold(),
        canon_path.display()
    );
    println!();

    // Read and parse the file
    let yaml_content = fs::read_to_string(&canon_path).map_err(CanonError::Io)?;

    // First try to parse as basic YAML to give better error messages
    let yaml_value: Value =
        serde_yaml::from_str(&yaml_content).map_err(|e| CanonError::ValidationError {
            message: format!("Invalid YAML syntax: {}", e),
        })?;

    // Now try to parse as a Canon specification
    let spec: CanonSpecification =
        serde_yaml::from_value(yaml_value.clone()).map_err(|e| CanonError::ValidationError {
            message: format!("Invalid Canon specification structure: {}", e),
        })?;

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate protocol version
    validate_protocol_version(&spec, &mut errors, &mut warnings);

    // Validate type reference
    validate_type_reference(&spec, &mut errors, &mut warnings);

    // Validate metadata
    validate_metadata(&spec, &mut errors, &mut warnings);

    // If this is a type definition, validate the schema
    if spec.r#type == "canon-protocol.org/type@1.0.0" {
        validate_type_definition(&spec, &mut errors, &mut warnings);
    } else {
        // Validate against the type's schema if we can fetch it
        validate_against_type(&spec, canon_path.parent(), &mut errors, &mut warnings).await;
    }

    // Report results
    report_validation_results(&canon_path, &errors, &warnings, strict)?;

    Ok(())
}

fn validate_protocol_version(
    spec: &CanonSpecification,
    errors: &mut Vec<String>,
    _warnings: &mut [String],
) {
    // Validate canon field format (should be like "1.0")
    if !spec.canon.chars().all(|c| c.is_ascii_digit() || c == '.') {
        errors.push(format!(
            "Invalid protocol version '{}': must be numeric (e.g., '1.0')",
            spec.canon
        ));
    }

    let parts: Vec<&str> = spec.canon.split('.').collect();
    if parts.len() != 2 {
        errors.push(format!(
            "Invalid protocol version '{}': must be MAJOR.MINOR format",
            spec.canon
        ));
    }
}

fn validate_type_reference(
    spec: &CanonSpecification,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Parse the type URI
    match Dependency::parse(&spec.r#type) {
        Ok(dep) => {
            // Check version is specified
            if dep.version.is_none() {
                warnings.push(format!(
                    "Type reference '{}' should include a version",
                    spec.r#type
                ));
            }

            // Check for version operators (not allowed in instance type references)
            if dep.version_operator.is_some() {
                errors.push(format!(
                    "Type reference '{}' cannot use version operators (^ or ~) in instances",
                    spec.r#type
                ));
            }
        }
        Err(e) => {
            errors.push(format!("Invalid type reference '{}': {}", spec.r#type, e));
        }
    }
}

fn validate_metadata(
    spec: &CanonSpecification,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Validate ID format (lowercase, alphanumeric with hyphens)
    if !spec
        .metadata
        .id
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        errors.push(format!(
            "Invalid metadata.id '{}': must be lowercase alphanumeric with hyphens only",
            spec.metadata.id
        ));
    }

    // Validate semantic version format
    if !is_valid_semver(&spec.metadata.version) {
        errors.push(format!(
            "Invalid metadata.version '{}': must be semantic version (MAJOR.MINOR.PATCH)",
            spec.metadata.version
        ));
    }

    // Validate publisher format (should be a domain)
    if !is_valid_publisher(&spec.metadata.publisher) {
        warnings.push(format!(
            "Publisher '{}' should be a valid domain or subdomain",
            spec.metadata.publisher
        ));
    }
}

fn validate_type_definition(
    spec: &CanonSpecification,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    if let Some(schema) = &spec.schema {
        for (field_name, field_def) in schema {
            validate_schema_field(field_name, field_def, errors, warnings);
        }
    }

    // Check includes for valid type references
    if let Some(includes) = &spec.includes {
        for include in includes {
            match Dependency::parse(include) {
                Ok(dep) => {
                    if dep.version.is_none() {
                        warnings.push(format!("Include '{}' should specify a version", include));
                    }
                }
                Err(e) => {
                    errors.push(format!("Invalid include '{}': {}", include, e));
                }
            }
        }
    }
}

fn validate_schema_field(
    field_name: &str,
    field: &SchemaField,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Validate field name (should be lowercase with underscores)
    if !field_name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        warnings.push(format!(
            "Field name '{}' should be lowercase with underscores",
            field_name
        ));
    }

    // Validate ref types have URI
    if matches!(field.r#type, FieldType::Ref) && field.uri.is_none() {
        errors.push(format!(
            "Field '{}' is type 'ref' but missing 'uri' property",
            field_name
        ));
    }

    // Validate pattern is only used with strings
    if field.pattern.is_some() && !matches!(field.r#type, FieldType::String) {
        warnings.push(format!(
            "Field '{}' has 'pattern' but is not type 'string'",
            field_name
        ));
    }

    // Validate properties is only used with objects
    if field.properties.is_some() && !matches!(field.r#type, FieldType::Object) {
        errors.push(format!(
            "Field '{}' has 'properties' but is not type 'object'",
            field_name
        ));
    }

    // Validate items is only used with arrays
    if field.items.is_some() && !matches!(field.r#type, FieldType::Array) {
        errors.push(format!(
            "Field '{}' has 'items' but is not type 'array'",
            field_name
        ));
    }

    // Recursively validate nested schemas
    if let Some(properties) = &field.properties {
        for (nested_name, nested_field) in properties {
            validate_schema_field(
                &format!("{}.{}", field_name, nested_name),
                nested_field,
                errors,
                warnings,
            );
        }
    }

    if let Some(items) = &field.items {
        validate_schema_field(&format!("{}[]", field_name), items, errors, warnings);
    }
}

async fn validate_against_type(
    spec: &CanonSpecification,
    base_dir: Option<&Path>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Parse the type URI
    let type_dep = match Dependency::parse(&spec.r#type) {
        Ok(dep) => dep,
        Err(_) => return, // Already reported in validate_type_reference
    };

    // Try to load the type definition
    let type_spec = match load_type_definition(&type_dep, base_dir).await {
        Ok(spec) => spec,
        Err(_) => {
            warnings.push(format!(
                "Could not load type definition '{}' for validation",
                spec.r#type
            ));
            return;
        }
    };

    // Validate the instance against the type's schema
    if let Some(schema) = &type_spec.schema {
        validate_instance_against_schema(&spec.content, schema, errors, warnings, "");
    }
}

async fn load_type_definition(
    dep: &Dependency,
    base_dir: Option<&Path>,
) -> CanonResult<CanonSpecification> {
    // First check if it's cached locally
    let local_path = if let Some(dir) = base_dir {
        dir.join(dep.local_path()).join("canon.yml")
    } else {
        dep.local_path().join("canon.yml")
    };

    if local_path.exists() {
        let content = fs::read_to_string(&local_path).map_err(CanonError::Io)?;
        let spec: CanonSpecification =
            serde_yaml::from_str(&content).map_err(|e| CanonError::ValidationError {
                message: format!("Failed to parse type definition: {}", e),
            })?;
        return Ok(spec);
    }

    // If not cached, we could fetch it from the registry
    // For now, we'll just return an error
    Err(CanonError::ValidationError {
        message: format!("Type definition not found locally: {}", dep.to_uri()),
    })
}

fn validate_instance_against_schema(
    content: &HashMap<String, Value>,
    schema: &HashMap<String, SchemaField>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
    path_prefix: &str,
) {
    // Check required fields
    for (field_name, field_def) in schema {
        let field_path = if path_prefix.is_empty() {
            field_name.clone()
        } else {
            format!("{}.{}", path_prefix, field_name)
        };

        if field_def.required.unwrap_or(false) && !content.contains_key(field_name) {
            errors.push(format!("Required field '{}' is missing", field_path));
            continue;
        }

        if let Some(value) = content.get(field_name) {
            validate_value_against_field(value, field_def, &field_path, errors, warnings);
        }
    }

    // Check for unknown fields (in strict mode)
    for field_name in content.keys() {
        if !schema.contains_key(field_name) {
            warnings.push(format!(
                "Unknown field '{}' not defined in schema",
                if path_prefix.is_empty() {
                    field_name.clone()
                } else {
                    format!("{}.{}", path_prefix, field_name)
                }
            ));
        }
    }
}

fn validate_value_against_field(
    value: &Value,
    field: &SchemaField,
    field_path: &str,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    // Type checking
    let type_valid = match field.r#type {
        FieldType::String => value.is_string(),
        FieldType::Number => value.is_number(),
        FieldType::Boolean => value.is_bool(),
        FieldType::Object => value.is_mapping(),
        FieldType::Array => value.is_sequence(),
        FieldType::Ref => value.is_string(), // Refs are strings with URIs
        FieldType::Any => true,
    };

    if !type_valid {
        errors.push(format!(
            "Field '{}' has wrong type: expected {:?}",
            field_path, field.r#type
        ));
        return;
    }

    // Additional validations based on type
    match field.r#type {
        FieldType::String => {
            if let Some(pattern) = &field.pattern {
                if let Some(s) = value.as_str() {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        if !re.is_match(s) {
                            errors.push(format!(
                                "Field '{}' value '{}' doesn't match pattern '{}'",
                                field_path, s, pattern
                            ));
                        }
                    }
                }
            }
        }
        FieldType::Ref => {
            if let Some(uri_str) = value.as_str() {
                match Dependency::parse(uri_str) {
                    Ok(dep) => {
                        // Instance refs must have exact versions
                        if dep.version_operator.is_some() {
                            errors.push(format!(
                                "Field '{}' reference '{}' cannot use version operators in instances",
                                field_path, uri_str
                            ));
                        }
                        if dep.version.is_none() {
                            warnings.push(format!(
                                "Field '{}' reference '{}' should specify an exact version",
                                field_path, uri_str
                            ));
                        }
                    }
                    Err(e) => {
                        errors.push(format!(
                            "Field '{}' has invalid reference '{}': {}",
                            field_path, uri_str, e
                        ));
                    }
                }
            }
        }
        FieldType::Array => {
            if let Some(sequence) = value.as_sequence() {
                if let Some(items_schema) = &field.items {
                    for (i, item) in sequence.iter().enumerate() {
                        validate_value_against_field(
                            item,
                            items_schema,
                            &format!("{}[{}]", field_path, i),
                            errors,
                            warnings,
                        );
                    }
                }
            }
        }
        FieldType::Object => {
            if let Some(mapping) = value.as_mapping() {
                if let Some(properties) = &field.properties {
                    let mut nested_content = HashMap::new();
                    for (k, v) in mapping {
                        if let Some(key_str) = k.as_str() {
                            nested_content.insert(key_str.to_string(), v.clone());
                        }
                    }
                    validate_instance_against_schema(
                        &nested_content,
                        properties,
                        errors,
                        warnings,
                        field_path,
                    );
                }
            }
        }
        _ => {}
    }

    // Check enum values
    if let Some(enum_values) = &field.r#enum {
        if !enum_values.contains(value) {
            errors.push(format!(
                "Field '{}' value must be one of: {:?}",
                field_path, enum_values
            ));
        }
    }
}

fn is_valid_semver(version: &str) -> bool {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u32>().is_ok())
}

fn is_valid_publisher(publisher: &str) -> bool {
    // Basic check for domain-like structure
    publisher.contains('.') || publisher == "localhost" || publisher == "example"
}

fn report_validation_results(
    path: &Path,
    errors: &[String],
    warnings: &[String],
    strict: bool,
) -> CanonResult<()> {
    let total_issues = errors.len() + warnings.len();

    if total_issues == 0 {
        println!("{} Specification is valid", style("✓").green().bold());
        println!();
        println!("  {} {}", style("✓").green(), path.display());
        return Ok(());
    }

    // Report errors
    if !errors.is_empty() {
        println!("{} {} errors found:", style("✗").red().bold(), errors.len());
        for error in errors {
            println!("  {} {}", style("✗").red(), error);
        }
        println!();
    }

    // Report warnings
    if !warnings.is_empty() {
        println!(
            "{} {} warnings found:",
            style("⚠").yellow().bold(),
            warnings.len()
        );
        for warning in warnings {
            println!("  {} {}", style("⚠").yellow(), warning);
        }
        println!();
    }

    // Summary
    if !errors.is_empty() || (strict && !warnings.is_empty()) {
        println!("{} Validation failed", style("✗").red().bold());
        Err(CanonError::ValidationError {
            message: format!("{} errors, {} warnings", errors.len(), warnings.len()),
        })
    } else {
        println!(
            "{} Validation passed with warnings",
            style("⚠").yellow().bold()
        );
        Ok(())
    }
}
