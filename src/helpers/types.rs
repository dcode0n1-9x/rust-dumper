use pricelevel::Side;
use pricelevel::TimeInForce;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteInstrument {
    pub instrumentToken: String,
}
#[derive(Debug, Deserialize)]
pub struct NewOrder {
    pub tradingSymbol: String,
    pub orderId: u64,
    pub price: i64,
    pub quantity: u64,
    pub side: Side,
    pub timeInForce: TimeInForce,
}

// {
//   id: "cmioqwl4c0001p0kes2jw7x8d",
//   orderId: "aaasad602606",
//   userId: "cmiomruj30002qwke536bg9ce",
//   instrumentId: "cmiomrupd002sqwkese0111tc",
//   parentOrderId: null,
//   exchangeOrderId: null,
//   exchangeTimestamp: null,
//   placedBy: "H",
//   variety: "REGULAR",
//   orderType: "MARKET",
//   transactionType: "BUY",
//   validity: "DAY",
//   product: "CNC",
//   exchange: "NSE",
//   tradingSymbol: "TATASTEEL",
//   quantity: 5,
//   disclosedQuantity: 0,
//   price: 5,
//   triggerPrice: 0,
//   averagePrice: 0,
//   filledQuantity: 0,
//   pendingQuantity: 5,
//   cancelledQuantity: 0,
//   status: "PENDING",
//   statusMessage: null,
//   tag: null,
//   clientOrderId: null,
//   orderTimestamp: 2025-12-02T15:41:10.763Z,
//   exchangeUpdateTime: null,
//   rejectedBy: null,
//   cancelledBy: null,
//   createdAt: 2025-12-02T15:41:10.763Z,
//   updatedAt: 2025-12-02T15:41:10.763Z,
//   modificationCount: 0,
// }

#[derive(Debug, Deserialize)]
pub enum EngineCommand {
    InstrumentCreate(InstrumentPayload),
    InstrumentDelete(DeleteInstrument),
    OrderCreate(NewOrder),
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
