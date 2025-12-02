use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
    pub group_id: String,
    pub topics: Vec<String>,
}

pub fn create_consumer(config: &KafkaConfig) -> Result<StreamConsumer, KafkaError> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &config.brokers)
        .set("group.id", &config.group_id)
        // .set("enable.partition.eof", "false")
        // .set("session.timeout.ms", "6000")
        // .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&config.topics.iter().map(String::as_str).collect::<Vec<_>>())?;

    Ok(consumer)
}
