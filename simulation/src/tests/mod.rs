use super::*;

// pub(crate) fn generate_config() -> SimulationConfig<Single> {
//     SimulationConfig { simulation: (), max_parallel: (), output_directory: (), output_file_name: (), agent_parameters: () }
// }

use std::{env, io::Read, path::Path};

#[test]
fn static_output() {
    simulations::batch("src/tests/configs/static.toml").unwrap();
    let path = Path::new(env::current_dir().unwrap().to_str().unwrap())
        .join("test_static_output/gbm_drift=0.1_vol=0.35/trajectory=0.json");
    println!("path: {:?}", path);
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    assert!(!contents.is_empty());
    std::fs::remove_dir_all("test_static").unwrap();
}

#[test]
fn sweep_output() {
    simulations::batch("configs/test/sweep.toml").unwrap();

    for drift in [-1, 1] {
        for vol in [0, 1] {
            for trajectory in [0, 1] {
                let str = format!(
                    "test_sweep/gbm_drift={}_vol={}/trajectory={}.json",
                    drift, vol, trajectory
                );
                let path = Path::new(env::current_dir().unwrap().to_str().unwrap()).join(str);
                println!("path: {:?}", path);
                let mut file = std::fs::File::open(path).unwrap();
                let mut contents = vec![];
                file.read_to_end(&mut contents).unwrap();
                assert!(!contents.is_empty());
            }
        }
    }

    // std::fs::remove_dir_all("test_sweep").unwrap();
}
