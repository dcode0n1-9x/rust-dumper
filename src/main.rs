mod config;
mod engine;
mod helpers;
mod orderbook;
mod utils;

use crate::config::kafka::{KafkaConfig, create_consumer};
use crate::helpers::{DeleteInstrument, EngineCommand, InstrumentCreateMessage, InstrumentPayload};
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
            "order.create".to_string(),
            "alert.create".to_string(),
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
                        match serde_json::from_str::<DeleteInstrument>(payload) {
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
                        match serde_json::from_str::<InstrumentCreateMessage>(payload) {
                            Ok(instr_msg) => {
                                let cmd = EngineCommand::InstrumentCreate(InstrumentPayload {
                                    id: instr_msg.instrument.id,
                                    instrumentToken: instr_msg.instrument.instrumentToken,
                                    exchangeToken: instr_msg.instrument.exchangeToken,
                                    tradingSymbol: instr_msg.instrument.tradingSymbol,
                                    name: instr_msg.instrument.name,
                                    exchange: instr_msg.instrument.exchange,
                                    segment: instr_msg.instrument.segment,
                                    instrumentType: instr_msg.instrument.instrumentType,
                                    tickSize: instr_msg.instrument.tickSize,
                                    lotSize: instr_msg.instrument.lotSize,
                                    expiry: instr_msg.instrument.expiry,
                                    strike: instr_msg.instrument.strike,
                                    isin: instr_msg.instrument.isin,
                                    isActive: instr_msg.instrument.isActive,
                                    lastPrice: instr_msg.instrument.lastPrice,
                                    lastUpdated: instr_msg.instrument.lastUpdated,
                                    createdAt: instr_msg.instrument.createdAt,
                                    updatedAt: instr_msg.instrument.updatedAt,
                                });
                                if let Err(e) = tx.send(cmd).await {
                                    warn!("Failed to send InstrumentCreate to engine: {}", e);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse instrument.create payload: {}", e);
                            } // let cmd = EngineCommand::InstrumentCreate(NewInstrument { tradingSymbol: (), instrument_token: (), name: (), exchange: (), segment: (), instrument_type: (), tick_size: (), lot_size: (), expiry: (), strike: (), isin: (), is_active: (), last_price: () });
                              // if let Err(e) = tx.send(cmd).await {
                              //     warn!("Failed to send InstrumentCreate to engine: {}", e);
                              // }
                        }
                    }
                    "order.create" => {
                        info!(
                            "[INFO] Received message on topic 'order.created': {}",
                            payload
                        );
                        // TODO: parse JSON/Protobuf here:
                        // This is just an example assuming JSON:
                        // { "symbol": "BTC/USD", "order_id": 123, "price": 50000, "quantity": 100, "side": "buy", "tif": "GTC" }

                        // if let Ok(order) = parse_new_order(payload) {
                        //     let cmd = EngineCommand::OrderCreate(order);
                        //     if let Err(e) = tx.send(cmd).await {
                        //         warn!("Failed to send OrderCreate to engine: {}", e);
                        //     }
                        // } else {
                        //     warn!("Failed to parse order.created payload: {}", payload);
                        // }
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
