use crate::utils::CanonResult;

pub async fn run_validate(
    _path: Option<String>,
    _strict: bool,
    _schema: Option<String>,
    _fix: bool,
) -> CanonResult<()> {
    println!("Validate command not yet implemented");
    Ok(())
}
