mod config;
mod engine;
mod helpers;
mod orderbook;
mod utils;
use crate::config::kafka::{KafkaConfig, create_consumer};
use crate::helpers::{
    DeleteInstrumentPayload, EngineCommand, InstrumentCreatePayload, OrderCancelPayload,
    OrderCreatePayload, OrderModifyPayload,
};
use futures::StreamExt;
use rdkafka::message::Message;
use tokio::sync::mpsc;
use tracing::{info, warn};

#[tokio::main]

async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    // 1) Engine command channel
    let (tx, rx) = mpsc::channel::<EngineCommand>(1024);
    // 2) Spawn engine task that owns BookManagerStd
    tokio::spawn(async move {
        engine::run_engine(rx).await;
    });
    // 3) Kafka consumer
    let kafka_config = KafkaConfig {
        brokers: "localhost:9092".to_string(),
        group_id: "orderbook_group".to_string(),
        topics: vec![
            "instrument.create".to_string(),
            "instrument.delete".to_string(),
            "alert.create".to_string(),
            "order.cancelled".to_string(),
            "order.create".to_string(),
            "order.modify".to_string(),
        ],
    };
    let consumer = create_consumer(&kafka_config).expect("Failed to create Kafka consumer");
    info!("[INFO] Kafka consumer created successfully");
    info!("[INFO] Subscribed to topics: {:?}", kafka_config.topics);
    info!("[INFO] Brokers: {}", kafka_config.brokers);
    let mut message_stream = consumer.stream();
    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(message) => {
                let topic = message.topic();
                let payload = message
                    .payload()
                    .and_then(|p| std::str::from_utf8(p).ok())
                    .unwrap_or("");
                match topic {
                    "alert.create" => {
                        info!(
                            "[INFO] Received message on topic 'alert.create': {}",
                            payload
                        );
                        // Currently ignoring alert messages
                    }
                    "instrument.delete" => {
                        match serde_json::from_str::<DeleteInstrumentPayload>(payload) {
                            Ok(delete_instr) => {
                                let cmd = EngineCommand::InstrumentDelete(delete_instr);
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send InstrumentDelete to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse instrument.delete payload: {}", e);
                            }
                        }
                    }
                    "instrument.create" => {
                        info!(
                            "[INFO] Received message on topic 'instrument.create': {}",
                            payload
                        );
                        match serde_json::from_str::<InstrumentCreatePayload>(payload) {
                            Ok(instr_msg) => {
                                let cmd =
                                    EngineCommand::InstrumentCreate(instr_msg);
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send InstrumentCreate to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse instrument.create payload: {}", e);
                            }
                        }
                    }
                    "order.create" => {
                        info!(
                            "[INFO] Received message on topic 'order.created': {}",
                            payload
                        );
                        match serde_json::from_str::<OrderCreatePayload>(payload) {
                            Ok(create_payload) => {
                                let cmd = EngineCommand::OrderCreate(create_payload);
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send OrderCreate to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse order.create payload: {}", e);
                            }
                        }
                    }
                    "order.cancelled" => {
                        info!(
                            "[INFO] Received message on topic 'order.cancelled': {}",
                            payload
                        );
                        match serde_json::from_str::<OrderCancelPayload>(payload) {
                            Ok(delete_order) => {
                                let cmd = EngineCommand::OrderCancel(delete_order);
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send OrderCancel to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse order.cancelled payload: {}", e);
                            }
                        }
                    }
                    "order.modify" => {
                        info!(
                            "[INFO] Received message on topic 'order.modify': {}",
                            payload
                        );
                        match serde_json::from_str::<OrderModifyPayload>(payload) {
                            Ok(modify_msg) => {
                                let cmd = EngineCommand::OrderModify(modify_msg);
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send OrderModify to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse order.modify payload: {}", e);
                            }
                        }
                    }
                    other => {
                        warn!("[WARN] Received message on unknown topic: {}", other);
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
    info!("[INFO] Stream ended or consumer disconnected");
}
