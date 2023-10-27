use std::{collections::BTreeMap, fs::File, io::BufReader};

use anyhow::Result;
use serde_json::{from_reader, Value};

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

    fn get_vectorized_events(
        &self,
        contract_name: &str,
        event_name: &str,
    ) -> BTreeMap<String, Vec<Value>> {
        let events = self.get_events(contract_name, event_name).unwrap();

        let mut map: BTreeMap<String, Vec<Value>> = BTreeMap::new();

        for event in events.iter() {
            if let Value::Object(event_obj) = event {
                for (key, value) in event_obj.iter() {
                    let entry = map.entry(key.clone()).or_default();
                    entry.push(value.clone());
                }
            }
        }

        map
    }
}

fn read_in_simulation_json(
    file_name: &str,
) -> Result<BTreeMap<String, BTreeMap<String, Vec<Value>>>, serde_json::Error> {
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);
    let map: BTreeMap<String, BTreeMap<String, Vec<Value>>> = from_reader(reader)?;
    Ok(map)
}

#[cfg(test)]
mod tests {
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
        assert_eq!(events.len(), 95);
        for index in 0..10 {
            println!("{:?}", events[index]);
        }
    }

    #[test]
    fn retrieve_vectorize_data() {
        let file_name = "test.json";
        let simulation_data = SimulationData::new(file_name).unwrap();
        let events = simulation_data.get_vectorized_events("g3m", "SwapFilter");

        for (key, values) in events.iter() {
            println!("{}: {:?}", key, values);
        }
    }
}
