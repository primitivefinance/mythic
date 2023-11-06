use super::*;

use std::{env, fs, io::Read, path::Path};

#[test]
fn static_output() {
    simulations::batch("src/tests/configs/static.toml").unwrap();
    let path = Path::new(env::current_dir().unwrap().to_str().unwrap())
        .join("src/tests/output/static/0.json");
    println!("path: {:?}", path);
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    assert!(!contents.is_empty());
    std::fs::remove_dir_all("src/tests/output/static/").unwrap();
}

#[test]
fn sweep_output() {
    simulations::batch("src/tests/configs/sweep.toml").unwrap();
    let entries = fs::read_dir("src/tests/output/sweep").unwrap();
    let count = entries.count();
    assert_eq!(count, 8193); // 2^13 + 1 given our current config parameter sets (+1 for the errors.json)
    std::fs::remove_dir_all("src/tests/output/sweep/").unwrap();
}
