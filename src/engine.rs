// src/engine.rs
use crate::helpers::EngineCommand;
use crate::helpers::instrument_helpers::{handle_instrument_create, handle_instrument_delete};
use crate::orderbook::manager::BookManagerStd;
use tokio::sync::mpsc::Receiver;
use tracing::info;

pub async fn run_engine(mut rx: Receiver<EngineCommand>) {
    let mut manager = BookManagerStd::<()>::new();
    // If your manager has this, start trade processor here:
    // let _processor_handle = manager.start_trade_processor();
    info!("Engine started, waiting for commands...");
    while let Some(cmd) = rx.recv().await {
        match cmd {
            EngineCommand::InstrumentCreate(instr) => {
                handle_instrument_create(&mut manager, instr);
            }

            EngineCommand::InstrumentDelete(delete_instr) => {
                handle_instrument_delete(&mut manager, delete_instr);
            }
            EngineCommand::OrderCreate(instr) => {
                crate::helpers::orderbooks_helpers::handle_order_create(&mut manager, instr);
            }
        }
    }
    info!("Engine stopped (command channel closed)");
}
