use crate::helpers::NewOrderPayload;
use crate::helpers::types::OrderDelete;
use crate::helpers::types::OrderModify;
use crate::orderbook::manager::BookManager;
use crate::orderbook::manager::BookManagerStd;
use pricelevel::{OrderId, OrderUpdate};
use tracing::{info, warn};

pub fn handle_order_create(manager: &mut BookManagerStd<()>, order: NewOrderPayload) {
    println!("Handling new order for symbol: {}", order.tradingSymbol);
    println!("Order details: {:?}", order);
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
        order.price,
        order.quantity,
        order.side,
        order.timeInForce,
        None,
    ) {
        warn!("Failed to add order {} on {}: {}", order_id, symbol, e);
    }
}

pub fn handle_order_delete(manager: &mut BookManagerStd<()>, order: OrderDelete) {
    let Some(book) = manager.get_book_mut(&order.tradingSymbol) else {
        warn!(
            "No book found for {}, cannot delete order {}",
            order.tradingSymbol, order.orderId
        );
        return;
    };
    let order_id = OrderId::from_u64(order.orderId);
    if let Err(e) = book.cancel_order(order_id) {
        warn!("Failed to remove order {} on {}: {}", order_id, order.tradingSymbol, e);
    }
}

pub fn handle_order_modify(
    manager: &mut BookManagerStd<()>,
    order : OrderModify
) {
    let Some(book) = manager.get_book_mut(&order.tradingSymbol) else {
        warn!(
            "No book found for {}, cannot modify order {}",
            order.tradingSymbol, order.orderId
        );
        return;
    };
    let order_id = OrderId::from_u64(order.orderId);
    let order_update = OrderUpdate::UpdatePriceAndQuantity {
        order_id,
        new_price: order.new_price,
        new_quantity: order.new_quantity,
    };
    if let Err(e) = book.update_order(order_update) {
        warn!("Failed to modify order {} on {}: {}", order_id, order.tradingSymbol, e);
    }
}
