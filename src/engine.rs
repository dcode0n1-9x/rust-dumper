// src/engine.rs
use crate::helpers::EngineCommand;
use crate::helpers::{handle_instrument_create, handle_instrument_delete, handle_order_create , handle_order_delete, handle_order_modify};
use crate::orderbook::manager::BookManagerStd;
use tokio::sync::mpsc::Receiver;
use tracing::info;

pub async fn run_engine(mut rx: Receiver<EngineCommand>) {
    let mut manager = BookManagerStd::<()>::new();
    info!("Engine started, waiting for commands...");
    while let Some(cmd) = rx.recv().await {
        match cmd {
            EngineCommand::InstrumentCreate(instr) => {
                handle_instrument_create(&mut manager, instr);
            }

            EngineCommand::InstrumentDelete(delete_instr) => {
                handle_instrument_delete(&mut manager, delete_instr);
            }
            EngineCommand::OrderCreate(order) => {
                handle_order_create(&mut manager, order);
            }
            EngineCommand::OrderDelete(order) => {
                handle_order_delete(&mut manager, order);
            }
            EngineCommand::OrderModify(order) => {
                handle_order_modify(&mut manager, order);
            }
        }
    }
    info!("Engine stopped (command channel closed)");
}
