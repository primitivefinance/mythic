use std::env;

use tokio::{fs, sync::mpsc, task};

use super::*;

/// `BatchData` is a struct that holds the simulation data and any errors that
/// occurred during the simulation.
///
/// # Fields
///
/// * `data` - A vector of `SimulationData` that holds the data from the
///   simulation.
/// * `errors` - A `Value` that holds any errors that occurred during the
///   simulation.
pub struct BatchData {
    pub data: Vec<SimulationData>,
    pub errors: Option<Value>,
}

impl BatchData {
    /// Creates a new `BatchData` instance by reading data from a specified
    /// directory.
    ///
    /// This function performs the following steps:
    /// 1. It gets the current working directory and appends the provided
    ///    directory path to it.
    /// 2. It creates two unbounded channels, one for data and one for errors.
    /// 3. It reads the directory and for each entry, it spawns a new task.
    /// 4. Each task reads the file content. If the file is "errors.json", it
    ///    sends the content to the errors channel. Otherwise, it creates a new
    ///    `SimulationData` instance from the file and sends it to the data
    ///    channel.
    /// 5. After reading all entries, it drops the senders of the channels.
    /// 6. It collects all received data and errors into vectors.
    /// 7. Finally, it returns a new `BatchData` instance containing the
    ///    collected data and errors.
    ///
    /// # Arguments
    ///
    /// * `dir` - A string slice that holds the name of the directory to read
    ///   the data from.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `BatchData` instance containing the read data and
    ///   errors.
    pub async fn new(dir: &str) -> Self {
        let cwd = env::current_dir().unwrap();
        let path = cwd.join(dir);
        println!("Reading `BatchData` from path: {:?}", path);

        let (data_sender, mut data_receiver) = mpsc::unbounded_channel();
        // let (errors_sender, mut errors_receiver) = mpsc::unbounded_channel();

        let mut handles = vec![];

        if let Ok(mut entries) = fs::read_dir(path).await {
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let data_sender = data_sender.clone();
                // let errors_sender = errors_sender.clone();

                handles.push(task::spawn(async move {
                    let path = entry.path();
                    // let file_name = path.file_name().unwrap().to_str().unwrap();
                    // if file_name == "errors.json" {
                    //     if let Ok(contents) = fs::read_to_string(&path).await {
                    //         if let Ok(json) = serde_json::from_str(&contents) {
                    //             errors_sender.send(json).unwrap();
                    //         }
                    //     }
                    // } else {
                    // TODO: Stream out K,V pairs
                    data_sender
                        .send(SimulationData::new(path.to_str().unwrap()).unwrap())
                        .unwrap();
                    debug!("Finished reading from file: {:?}", path);
                    // }
                }));
            }
            // Now there are no new entries, we can drop the sender
            drop(data_sender);
            // drop(errors_sender);
        }
        let mut data = vec![];
        while let Some(data_recv) = data_receiver.recv().await {
            data.push(data_recv);
        }

        // let errors = errors_receiver.recv().await.unwrap();

        Self { data, errors: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing_test::traced_test]
    #[tokio::test(flavor = "multi_thread")]
    async fn new() {
        let batch = BatchData::new("src/tests/output").await;
        assert_eq!(batch.data.len(), 2);
        assert!(batch.errors.unwrap().is_array());
    }
}
