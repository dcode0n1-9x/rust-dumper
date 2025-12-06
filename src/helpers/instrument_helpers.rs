use super::{DeleteInstrumentPayload, InstrumentCreatePayload};
use crate::orderbook::manager::BookManager;
use crate::orderbook::manager::BookManagerStd;
use tracing::{info, warn};

pub fn handle_instrument_create(manager: &mut BookManagerStd<()>, instr: InstrumentCreatePayload) {
    let token = instr.instrumentToken;
    println!("Handling instrument create for symbol: {}", token);
    if manager.get_book(&token).is_some() {
        warn!("Instrument {} already exists, skipping", token);
        return;
    }
    info!("Creating new order book for {}", token);
    manager.add_book(&token);
}

pub fn handle_instrument_delete(
    manager: &mut BookManagerStd<()>,
    delete_instr: DeleteInstrumentPayload,
) {
    let instrument_id = delete_instr.instrumentToken;
    if manager.get_book(&instrument_id).is_none() {
        warn!("Instrument {} does not exist, cannot delete", instrument_id);
        return;
    }
    info!("Deleting order book for {}", instrument_id);
    manager.remove_book(&instrument_id);
}
