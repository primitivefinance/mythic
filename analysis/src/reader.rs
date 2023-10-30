use serde::de::DeserializeOwned;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationData(pub BTreeMap<String, BTreeMap<String, Vec<Value>>>);

impl SimulationData {
    pub fn new(file_name: &str) -> Result<Self, serde_json::Error> {
        let map = read_in_simulation_json(file_name)?;
        Ok(Self(map))
    }

    pub fn get_events(&self, contract_name: &str, event_name: &str) -> Option<&Vec<Value>> {
        self.0
            .get(contract_name)
            .and_then(|events| events.get(event_name))
    }

    pub fn get_vectorized_events<T>(&self, contract_name: &str) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        let mut results = Vec::new();

        // Get the map of events for the given contract name
        if let Some(events_map) = self.0.get(contract_name) {
            for (_, event_values) in events_map.iter() {
                // For each event value, attempt to deserialize it into type T
                for value in event_values.iter() {
                    if let Ok(deserialized_value) = serde_json::from_value::<T>(value.clone()) {
                        results.push(deserialized_value);
                    }
                }
            }
        }
        results
    }
}

use std::{env, path::PathBuf};

fn read_in_simulation_json(
    file_name: &str,
) -> Result<BTreeMap<String, BTreeMap<String, Vec<Value>>>, serde_json::Error> {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));
    let abs_path = current_dir.join(file_name);

    let file =
        File::open(&file_name).expect(&format!("Failed to open file at: {:?}", abs_path.display()));

    let reader = BufReader::new(file);
    let map: BTreeMap<String, BTreeMap<String, Vec<Value>>> = from_reader(reader)?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use simulation::bindings::g3m::{G3MErrors, G3MEvents};

    use super::*;

    #[test]
    fn read_json_to_simulation_data() {
        let file_name = "test.json";
        let result = read_in_simulation_json(file_name);
        assert!(result.is_ok());
    }

    #[test]
    fn retrieve_events() {
        let file_name = "test.json";
        let simulation_data = SimulationData::new(file_name).unwrap();
        let events = simulation_data.get_events("g3m", "SwapFilter");
        assert!(events.is_some());
        let events = events.unwrap();
        assert_eq!(events.len(), 131);
        for index in 0..10 {
            println!("{:?}", events[index]);
        }
    }

    #[test]
    fn retrieve_vectorized_event_structs() {
        let file_name = "test.json";
        let simulation_data = SimulationData::new(file_name).unwrap();
        let values = simulation_data.get_vectorized_events::<g3m::SwapFilter>("g3m");
        assert_eq!(values.len(), 131);
        for index in 0..10 {
            println!("{:?}", values[index]);
        }
    }
}
