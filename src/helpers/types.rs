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

#[derive(Debug, Deserialize)]
pub struct DeleteInstrumentPayload {
    pub instrumentToken: String,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentCreatePayload {
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
pub struct OrderCreatePayload {
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
pub struct OrderCancelPayload {
    pub orderId: u64,
    pub tradingSymbol: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderModifyPayload {
    pub tradingSymbol: String,
    pub orderId: u64,
    pub new_price: u64,
    pub new_quantity: u64,
}