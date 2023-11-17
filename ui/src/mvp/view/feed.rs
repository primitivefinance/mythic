//! View components for rendering feeds of data.

use std::collections::{HashMap, VecDeque};

use iced::{
    widget::{column, *},
    Element,
};

use super::{tracer::*, view::Message, *};

pub struct Feed {
    pub buckets: HashMap<String, VecDeque<AppEventMetadata>>,
    pub max_size: usize,
}

impl Feed {
    pub fn new(max_size: usize) -> Feed {
        Feed {
            buckets: HashMap::new(),
            max_size,
        }
    }

    /// Buckets the logs into the appropriate buckets.
    pub fn vec_to_bucketed_logs(&mut self, logs: VecDeque<AppEventLog>) {
        let (parsed, names) = filter_to_buckets(logs);
        for (name, bucket) in names.iter().zip(parsed.iter()) {
            self.buckets.insert(name.clone(), bucket.clone());
        }
    }

    /// Convert to data.
    pub fn convert_to_data(&self, bucket: &str) -> VecDeque<String> {
        let logs = self.buckets.get(bucket).unwrap();
        convert_metadata_to_data(logs.clone())
    }

    /// Renders the feed.
    pub fn view<'a>(&self, bucket: &str) -> Element<'a, Message> {
        let parsed = self.convert_to_data(bucket).clone();
        finite_firehose(parsed.clone(), bucket.to_string()).into()
    }
}

/// Input = Vector of AppEventLogs
/// Output = Buckets of AppEventMetdata
fn filter_to_buckets(
    logs: VecDeque<AppEventLog>,
) -> (Vec<VecDeque<AppEventMetadata>>, Vec<String>) {
    let mut system_logs = VecDeque::new();
    let mut user_logs = VecDeque::new();
    let mut world_logs: HashMap<u32, VecDeque<AppEventMetadata>> = HashMap::new();
    let mut agent_logs = VecDeque::new();
    let mut default_logs = VecDeque::new();
    let mut world_names: Vec<String> = vec![];

    // Iterate over each log in the VecDeque
    for log in logs {
        if let Some(metadata) = log.data.get(&AppEventLayer::System) {
            system_logs.push_back(metadata.clone());
        }
        if let Some(metadata) = log.data.get(&AppEventLayer::User) {
            user_logs.push_back(metadata.clone());
        }
        if let Some(metadata) = log.data.get(&AppEventLayer::World) {
            let world_id = metadata.id;
            world_logs
                .entry(world_id)
                .or_insert_with(VecDeque::new)
                .push_back(metadata.clone());
            world_names.push(world_id.to_string());
        }
        if let Some(metadata) = log.data.get(&AppEventLayer::Agent) {
            agent_logs.push_back(metadata.clone());
        }
        if let Some(metadata) = log.data.get(&AppEventLayer::Default) {
            default_logs.push_back(metadata.clone());
        }
    }

    let mut result = vec![system_logs, user_logs, default_logs, agent_logs];
    for (_, world_log) in world_logs {
        result.push(world_log);
    }

    (
        result,
        vec![
            "system".to_string(),
            "user".to_string(),
            "default".to_string(),
            "agent".to_string(),
        ]
        .into_iter()
        .chain(world_names.into_iter())
        .collect::<Vec<String>>(),
    )
}

pub fn convert_metadata_to_data(metadata: VecDeque<AppEventMetadata>) -> VecDeque<String> {
    metadata
        .iter()
        .map(|metadata| {
            metadata
                .data
                .last()
                .unwrap_or(&"No data".to_string())
                .clone()
        })
        .collect::<VecDeque<String>>()
}

/// Render an individual firehose log.
pub fn firehose_log<'a>(log: String) -> Element<'a, Message> {
    let firehose_element = text(log)
        .size(12)
        .vertical_alignment(alignment::Vertical::Center)
        .horizontal_alignment(alignment::Horizontal::Left);

    container(firehose_element)
        .style(FirehoseTrace::theme())
        .width(Length::Fill)
        .padding(4)
        .into()
}

/// Renders a column of firehose logs.
pub fn finite_firehose<'a>(logs: VecDeque<String>, label: String) -> Element<'a, Message> {
    let mut content = Column::new()
        .push(Text::new(label).size(24))
        .max_width(300.0);

    // The last log should go to the top of the firehose.
    let feed = logs.iter().rev().fold(column![].spacing(2), |column, log| {
        column.push(firehose_log(log.clone()))
    });

    let feed_container = container(feed)
        .height(Length::Fixed(400.0))
        .width(Length::Fixed(300.0))
        .style(FirehoseContainer::theme());

    let expand_button = button(text("View full"))
        .width(Length::Fill)
        .on_press(Message::Empty);

    content = content.push(feed_container).push(expand_button);
    content.padding(16).spacing(16).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_feed() {
        //  let mut feed = Feed::new(2);
        //
        // let mut data: HashMap<AppEventLayer, AppEventMetadata> =
        // HashMap::new(); let log: AppEventMetadata = AppEventMetadata
        // { id: 0,
        // data: vec!["test".to_string()],
        // action: AppEventAction::Add,
        // };
        //
        // data.insert(AppEventLayer::Agent, log);
        //
        // assert_eq!(feed.buckets.len(), 1);
        // assert_eq!(feed.buckets.get("test").unwrap().len(), 1);
    }
}
