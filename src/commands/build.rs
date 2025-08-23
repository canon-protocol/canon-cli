use crate::utils::CanonResult;

pub async fn run_build(
    _engine: Option<String>,
    _output: Option<String>,
    _sign: bool,
    _key: Option<String>,
    _no_cache: bool,
    _parallel: Option<usize>,
) -> CanonResult<()> {
    println!("Build command not yet implemented");
    Ok(())
}
