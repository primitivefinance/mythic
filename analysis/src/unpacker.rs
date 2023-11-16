use std::env;

use tokio::{fs, sync::mpsc, task};

use super::*;

pub struct BatchData {
    pub data: Vec<SimulationData>,
    pub errors: Value,
}

impl BatchData {
    pub async fn new(dir: &str) -> Self {
        let cwd = env::current_dir().unwrap();
        let path = cwd.join(dir);
        info!("Reading `BatchData` from path: {:?}", path);

        let (data_sender, mut data_receiver) = mpsc::unbounded_channel();
        let (errors_sender, mut errors_receiver) = mpsc::unbounded_channel();

        let mut handles = vec![];

        if let Ok(mut entries) = fs::read_dir(path).await {
            while let Some(entry) = entries.next_entry().await.unwrap() {
                let data_sender = data_sender.clone();
                let errors_sender = errors_sender.clone();

                handles.push(task::spawn(async move {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if file_name == "errors.json" {
                        if let Ok(contents) = fs::read_to_string(&path).await {
                            if let Ok(json) = serde_json::from_str(&contents) {
                                errors_sender.send(json).unwrap();
                            }
                        }
                    } else {
                        data_sender
                            .send(SimulationData::new(path.to_str().unwrap()).unwrap())
                            .unwrap();
                        debug!("Finished reading from file: {:?}", path);
                    }
                }));
            }
            // Now there are no new entries, we can drop the sender
            drop(data_sender);
            drop(errors_sender);
        }
        let mut data = vec![];
        while let Some(data_recv) = data_receiver.recv().await {
            data.push(data_recv);
        }

        let errors = errors_receiver.recv().await.unwrap();

        Self { data, errors }
    }

    fn organize(self) -> Self {
        // Idea, stream in all the metadata and in a concurrent process build up a filtering for it so we can group them into different parameter settings.
        for data in self.data.iter() {
            let metadata = data.metadata.as_ref().unwrap();
            print!("metadata: {:?}", data.metadata);
        }

        // TODO: Note the metadata is a simulation config so we can use it to filter over?
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing_test::traced_test]
    #[tokio::test(flavor = "multi_thread")]
    async fn new() {
        let now = std::time::Instant::now();
        let batch = BatchData::new("dca/sweep").await;
        assert_eq!(batch.data.len(), 100);
        assert!(batch.errors.is_array());
        let duration = now.elapsed();
        println!("Duration: {:?}", duration);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn organize() {
        let batch = BatchData::new("dca/sweep").await;
        let batch = batch.organize();
        assert_eq!(batch.data.len(), 100);
    }
}

// TODO: Break apart into different distinct configs
