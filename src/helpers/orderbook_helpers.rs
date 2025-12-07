use super::{OrderCancelPayload, OrderCreatePayload, OrderModifyPayload};
use crate::helpers::types::OrderType;
use crate::orderbook::manager::BookManager;
use crate::orderbook::manager::BookManagerStd;
use pricelevel::{OrderId, OrderUpdate, Side};
use tracing::{info, warn};
pub fn handle_order_create(manager: &mut BookManagerStd<()>, order: OrderCreatePayload) {
    let symbol = order.instrument_id.clone();
    if manager.get_book(&symbol).is_none() {
        info!("No book for {}, creating one on demand", symbol);
        manager.add_book(&symbol);
    }
    let Some(book) = manager.get_book(&symbol) else {
        warn!("Unable to get book for {} after add_book", symbol);
        return;
    };

    let order_id = OrderId::from_u64(order.order_id);

    match order.order_type {
        // If it's a market order (you may use a flag in payload to distinguish); replace the check if different.
        // I assume payload has enough info; if you use separate endpoint for market
        OrderType::MARKET /* replace with order.is_market */ => {
            match book.submit_market_order(order_id, order.quantity, order.side) {
                Ok(match_result) => {
                    info!("Market order {} matched on {}", order_id, symbol);
                    info!(
                        "Market order {} executed quantity {} on {}",
                        order_id,
                        match_result.executed_quantity(),
                        symbol
                    );
                }
                Err(e) => {
                    warn!("Market match failed for {} on {}: {}", order_id, symbol, e);
                }
            }
        }
        OrderType::LIMIT /* limit order */ => {
            // Check whether order is aggressive (crosses book)
            let should_attempt_match = match order.side {
                Side::Buy => {
                    if let Some(best_ask) = book.best_ask() {
                        order.price >= best_ask
                    } else {
                        false
                    }
                }
                Side::Sell => {
                    if let Some(best_bid) = book.best_bid() {
                        order.price <= best_bid
                    } else {
                        false
                    }
                }
            };

            if should_attempt_match {
                // Try matching first
                match book.match_limit_order(order_id, order.quantity, order.side, order.price) {
                    Ok(match_result) => {
                        // If there were transactions, trade_listener already invoked inside match_limit_order
                        info!("Limit order {} partially/fully matched: executed {} on {}", order_id, match_result.executed_quantity(), symbol);
                        if match_result.remaining_quantity > 0 {
                            // Add remaining as a resting order
                            if let Err(e) = book.add_limit_order(
                                order_id,
                                order.price,
                                match_result.remaining_quantity,
                                order.side,
                                order.time_in_force,
                                None,
                            ) {
                                warn!("Failed to add leftover resting order {} on {}: {}", order_id, symbol, e);
                            } else {
                                info!("Added resting remainder {} qty for order {} on {}", match_result.remaining_quantity, order_id, symbol);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Matching failed for limit {} on {}: {}", order_id, symbol, e);
                        // Fallback: insert as resting order
                        if let Err(e2) = book.add_limit_order(
                            order_id,
                            order.price,
                            order.quantity,
                            order.side,
                            order.time_in_force,
                            None,
                        ) {
                            warn!("Failed to add order {} after match failure: {}", order_id, e2);
                        }
                    }
                }
            } else {
                // Not aggressive -> insert as resting order directly
                if let Err(e) = book.add_limit_order(
                    order_id,
                    order.price,
                    order.quantity,
                    order.side,
                    order.time_in_force,
                    None,
                ) {
                    warn!("Failed to add order {} on {}: {}", order_id, symbol, e);
                } else {
                    info!("Added order {} on {}", order_id, symbol);
                }
            }
        }
    }
}

pub fn handle_order_cancel(manager: &mut BookManagerStd<()>, order: OrderCancelPayload) {
    let Some(book) = manager.get_book_mut(&order.instrument_id) else {
        warn!(
            "No book found for {}, cannot cancel order {}",
            order.instrument_id, order.order_id
        );
        return;
    };
    let order_id = OrderId::from_u64(order.order_id);
    if let Err(e) = book.cancel_order(order_id) {
        warn!(
            "Failed to remove order {} on {}: {}",
            order_id, order.instrument_id, e
        );
    }
    info!("Cancelled order {} on {}", order_id, order.instrument_id);
}

pub fn handle_order_modify(manager: &mut BookManagerStd<()>, order: OrderModifyPayload) {
    let Some(book) = manager.get_book_mut(&order.instrument_id) else {
        warn!(
            "No book found for {}, cannot modify order {}",
            order.instrument_id, order.order_id
        );
        return;
    };
    let order_id = OrderId::from_u64(order.order_id);
    let order_update = OrderUpdate::UpdatePriceAndQuantity {
        order_id,
        new_price: order.price,
        new_quantity: order.quantity,
    };
    if let Err(e) = book.update_order(order_update) {
        warn!(
            "Failed to modify order {} on {}: {}",
            order_id, order.instrument_id, e
        );
    }
}
