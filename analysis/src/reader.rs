use std::{env, path::PathBuf};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use simulation::{
    settings::{parameters::Single, SimulationConfig},
    simulations::Simulation,
};

use super::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SimulationData {
    #[serde(rename = "events")]
    pub contract_events: BTreeMap<String, BTreeMap<String, Vec<Value>>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<SimulationConfig<Single>>,
}

impl SimulationData {
    pub fn new(file_name: &str) -> Result<SimulationData> {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(""));
        let abs_path = current_dir.join(file_name);

        let file = File::open(file_name)
            .unwrap_or_else(|_| panic!("Failed to open file at: {:?}", abs_path.display()));

        let reader = BufReader::new(file);
        let data: SimulationData = from_reader(reader)?;
        Ok(data)
    }

    pub fn get_events(&self, contract_name: &str, event_name: &str) -> Option<&Vec<Value>> {
        self.contract_events
            .get(contract_name)
            .and_then(|events| events.get(event_name))
    }

    pub fn get_vectorized_events_from_str(
        &self,
        contract_name: &str,
        event_name: &str,
    ) -> BTreeMap<String, Vec<Value>> {
        let event = self.get_events(contract_name, event_name).unwrap();
        let mut vectorized_events: BTreeMap<String, Vec<Value>> = BTreeMap::new();

        // Assuming event is a Map<String, Value>
        for map in event.iter().map(|x| x.as_object().unwrap()) {
            for (key, value) in map {
                vectorized_events
                    .entry(key.clone())
                    .or_default()
                    .push(value.clone());
            }
        }

        vectorized_events
    }

    pub fn get_vectorized_events<T>(&self, contract_name: &str) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        let mut results = Vec::new();

        // Get the map of events for the given contract name
        if let Some(events_map) = self.contract_events.get(contract_name) {
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

#[cfg(test)]
mod tests {
    use simulation::{
        bindings::g3m::{G3MErrors, G3MEvents},
        simulations::SimulationType,
    };

    use super::*;

    const FILE_NAME: &str = "src/tests/output/0.json";

    #[test]
    fn read_json_to_simulation_data() {
        let simulation_data = SimulationData::new(FILE_NAME);
        assert!(simulation_data.is_ok());
    }

    #[test]
    fn retrieve_events() {
        let simulation_data = SimulationData::new(FILE_NAME).unwrap();
        let events = simulation_data.get_events("g3m", "SwapFilter");
        assert!(events.is_some());
        let events = events.unwrap();
        assert_eq!(events.len(), 8);
    }

    #[test]
    fn retrieve_vectorized_event_structs() {
        let simulation_data = SimulationData::new(FILE_NAME).unwrap();
        let values = simulation_data.get_vectorized_events::<g3m::SwapFilter>("g3m");
        assert_eq!(values.len(), 8);
    }

    #[test]
    fn retrieve_vectorized_event_structs_from_str() {
        let simulation_data = SimulationData::new(FILE_NAME).unwrap();
        let values = simulation_data.get_vectorized_events_from_str("g3m", "SwapFilter");
        assert_eq!(values.get("sender").unwrap().len(), 8);
    }

    #[test]
    fn retrieve_metadata() {
        let simulation_data = SimulationData::new(FILE_NAME).unwrap();
        let metadata = simulation_data.metadata.as_ref();
        assert!(metadata.is_some());
    }
}
