use std::path::{Path, PathBuf};

pub fn get_artifact_path(name: &str) -> PathBuf {
    // counter -> Counter
    let contract_name = name.chars().next().unwrap().to_uppercase().to_string() + &name[1..];
    // ui/
    let cwd = std::env::current_dir().unwrap();
    // root/contracts/out/Counter.sol/Counter.json
    // todo: clean this up...
    Path::new(&cwd)
        .join("contracts")
        .join("out")
        .join(format!("{}.sol", contract_name))
        .join(format!("{}.json", contract_name))
}

/// Hack to trigger Command messages.
pub async fn empty_async() -> Result<(), ()> {
    Ok(())
}
