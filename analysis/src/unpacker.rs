use std::{env, fs, path::PathBuf};

use linked_hash_map::LinkedHashMap;

use super::*;

pub struct BatchData(pub (Vec<SimulationData>, Value));

impl BatchData {
    pub fn new(dir: &str) -> Self {
        let cwd = env::current_dir().unwrap();
        let path = cwd.join(dir);
        println!("Path is: {:?}", path);
        let mut data = Vec::new();
        let mut errors: Option<Value> = None;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name == "errors.json" {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        if let Ok(json) = serde_json::from_str(&contents) {
                            errors = Some(json);
                        }
                    }
                    continue;
                }

                let path = entry.path();
                println!("File path: {:?}", path);
                data.push(SimulationData::new(path.to_str().unwrap()).unwrap());
            }
        }
        println!("errors: {:?}", errors);
        Self((data, errors.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let batch_data = BatchData::new("dca/sweep");
        assert_eq!(batch_data.0 .0.len(), 100);
        assert!(batch_data.0 .1.is_array());
    }
}
