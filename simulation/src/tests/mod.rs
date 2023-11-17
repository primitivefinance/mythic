use std::{env, fs, io::Read, path::Path};

use super::*;

#[test]
fn static_output() {
    let config_path = "src/tests/configs/static.toml";
    let config = simulations::import(config_path).unwrap();
    simulations::batch(config).unwrap();
    let path = Path::new(env::current_dir().unwrap().to_str().unwrap())
        .join("src/tests/output/static/0.json");
    println!("path: {:?}", path);
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    assert!(!contents.is_empty());
}

#[test]
fn sweep_output() {
    let config_path = "src/tests/configs/sweep.toml";
    let config = simulations::import(config_path).unwrap();
    simulations::batch(config).unwrap();
    let entries = fs::read_dir("src/tests/output/sweep").unwrap();
    let count = entries.count();
    assert_eq!(count, 129); // 2^7 + 1 given our current config parameter sets (+1 for the errors.json)
}
