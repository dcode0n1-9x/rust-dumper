use pricelevel::Side;
use pricelevel::TimeInForce;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum EngineCommand {
    InstrumentCreate(InstrumentPayload),
    InstrumentDelete(DeleteInstrument),
    OrderCreate(NewOrderPayload),
    OrderDelete(OrderDelete),
    OrderModify(OrderModify),
}

#[derive(Debug, Deserialize)]
pub struct DeleteInstrument {
    pub instrumentToken: String,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentPayload {
    pub id: String,
    pub instrumentToken: String,
    pub exchangeToken: String,
    pub tradingSymbol: String,
    pub name: String,
    pub exchange: String,
    pub segment: String,
    pub instrumentType: String,
    pub tickSize: f64,
    pub lotSize: i32,
    pub expiry: Option<String>,
    pub strike: Option<f64>,
    pub isin: Option<String>,
    pub isActive: bool,
    pub lastPrice: f64,
    pub lastUpdated: String,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentCreateMessage {
    pub instrument: InstrumentPayload,
}

#[derive(Debug, Deserialize)]


pub struct NewOrderMessage {
    pub order: NewOrderPayload,
}

#[derive(Debug, Deserialize)]
pub struct NewOrderPayload {
    pub orderId: u64,
    pub userId: String,
    pub instrumentId: String,
    pub tradingSymbol: String,
    pub exchange: String,
    pub product: String,
    pub variety: String,
    pub orderType: String,
    pub transactionType: String,
    pub quantity: u64,
    pub price: u64,
    pub side: Side,
    pub timeInForce: TimeInForce,
}

#[derive(Debug, Deserialize)]
pub struct OrderDelete {
    pub tradingSymbol: String,
    pub orderId: u64,
}

#[derive(Debug, Deserialize)]
pub struct OrderModify {
    pub tradingSymbol: String,
    pub orderId: u64,
    pub new_price: u64,
    pub new_quantity: u64,
}