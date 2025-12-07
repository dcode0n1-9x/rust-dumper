use pricelevel::Side;
use pricelevel::TimeInForce;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EngineCommand {
    InstrumentCreate(InstrumentCreatePayload),
    InstrumentDelete(DeleteInstrumentPayload),
    OrderCreate(OrderCreatePayload),
    OrderCancel(OrderCancelPayload),
    OrderModify(OrderModifyPayload),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OrderType {
    MARKET,
    LIMIT,
}
#[derive(Debug, Deserialize)]
pub struct DeleteInstrumentPayload {
    pub instrument_id: String,
}


#[derive(Debug, Deserialize)]
pub struct InstrumentCreatePayload {
    pub instrument_id: String,
}
#[derive(Debug, Deserialize)]
pub struct OrderCreatePayload {
    pub order_id: u64,
    pub instrument_id: String,
    pub quantity: u64,
    pub price: u64,
    pub side: Side,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
}
#[derive(Debug, Deserialize)]
pub struct OrderCancelPayload {
    pub order_id: u64,
    pub instrument_id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderModifyPayload {
    pub instrument_id: String,
    pub order_id: u64,
    pub price: u64,
    pub quantity: u64,
}
