mod config;
mod helpers;

use crate::config::kafka::{KafkaConfig, create_consumer};

use futures::StreamExt;
use rdkafka::message::Message;

#[tokio::main]
async fn main() {
    let kafka_config = KafkaConfig {
        brokers: "localhost:9092".to_string(),
        group_id: "orderbook_group".to_string(),
        topics: vec!["instrument.create".to_string(), "order.created".to_string()],
    };
    let consumer = create_consumer(&kafka_config).expect("Failed to create Kafka consumer");

    println!("[INFO] Kafka consumer created successfully");
    println!("[INFO] Subscribed to topics: {:?}", kafka_config.topics);
    println!("[INFO] Brokers: {}", kafka_config.brokers);

    let mut message_stream = consumer.stream();

    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(message) => {
                let topic: &str = message.topic();
                if topic == "instrument.create" {
                    println!("[INFO] Received message on topic 'instrument.create'");
                } else if topic == "order.created" {
                    println!("[INFO] Received message on topic 'order.created'");
                } else {
                    println!("[WARN] Received message on unknown topic: {}", topic);
                }
                if let Some(payload) = message.payload() {
                    match std::str::from_utf8(payload) {
                        Ok(s) => println!("Received message payload: {}", s),
                        Err(_) => println!("Non-UTF8 message payload"),
                    }
                } else {
                    println!("Received message with empty payload");
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }

    println!("[INFO] Stream ended or consumer disconnected");
}
