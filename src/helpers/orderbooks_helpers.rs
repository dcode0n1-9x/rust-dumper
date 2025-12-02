use crate::helpers::NewOrder;
use crate::orderbook::manager::BookManager;
use crate::orderbook::manager::BookManagerStd;
use pricelevel::OrderId;
use tracing::{info, warn};

pub fn handle_order_create(manager: &mut BookManagerStd<()>, order: NewOrder) {
    let symbol = order.tradingSymbol;
    if manager.get_book(&symbol).is_none() {
        info!("No book for {}, creating one on demand", symbol);
        manager.add_book(&symbol);
    }
    let Some(book) = manager.get_book(&symbol) else {
        warn!("Unable to get book for {} after add_book", symbol);
        return;
    };
    let order_id = OrderId::from_u64(order.orderId);
    if let Err(e) = book.add_limit_order(
        order_id,
        order.price as u64,
        order.quantity,
        order.side,
        order.timeInForce,
        None,
    ) {
        warn!("Failed to add order {} on {}: {}", order_id, symbol, e);
    }
}
