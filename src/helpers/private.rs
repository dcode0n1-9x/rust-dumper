use super::book_change_event::PriceLevelChangedEvent;
use super::book::OrderBook;
use super::error::OrderBookError;
use crate::utils::time::current_time_millis;
use pricelevel::{OrderType, PriceLevel, Side};
use std::sync::Arc;
use std::sync::atomic::Ordering;

impl<T> OrderBook<T>
where
    T: Clone + Send + Sync + Default + 'static,
{
    /// Check if an order has expired
    pub fn has_expired(&self, order: &OrderType<T>) -> bool {
        let time_in_force = order.time_in_force();
        let current_time = current_time_millis();

        // Only check market close timestamp if we have one set
        let market_close = if self.has_market_close.load(Ordering::Relaxed) {
            Some(self.market_close_timestamp.load(Ordering::Relaxed))
        } else {
            None
        };

        time_in_force.is_expired(current_time, market_close)
    }

    /// Check if there would be a price crossing
    pub fn will_cross_market(&self, price: u64, side: Side) -> bool {
        match side {
            Side::Buy => OrderBook::<T>::best_ask(self).is_some_and(|best_ask| price >= best_ask),
            Side::Sell => OrderBook::<T>::best_bid(self).is_some_and(|best_bid| price <= best_bid),
        }
    }

    /// Places a resting order in the book, updates its location.
    #[allow(dead_code)]
    pub fn place_order_in_book(
        &self,
        order: Arc<OrderType<T>>,
    ) -> Result<Arc<OrderType<T>>, OrderBookError> {
        let (side, price, order_id) = (order.side(), order.price(), order.id());

        
        let book_side = match side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        };

        // Get or create the price level
        let price_level = book_side
            .get_or_insert(price, Arc::new(PriceLevel::new(price)))
            .value()
            .clone();

        // Convert OrderType<T> to OrderType<()> for compatibility with current PriceLevel API
        let unit_order = self.convert_to_unit_type(&*order);
        let _added_order = price_level.add_order(unit_order);
        
        // notify price level changes
        if let Some(ref listener) = self.price_level_changed_listener {
            listener(PriceLevelChangedEvent {
                side,
                price: price_level.price(),
                quantity: price_level.visible_quantity(),
            })
        }
        // The location is stored as (price, side) for efficient retrieval in cancel_order
        self.order_locations.insert(order_id, (price, side));

        Ok(order)
    }

    /// Convert `OrderType<T>` to OrderType<()> for compatibility with current PriceLevel API
    pub fn convert_to_unit_type(&self, order: &OrderType<T>) -> OrderType<()> {
        match order {
            OrderType::Standard {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
                ..
            } => OrderType::Standard {
                id: *id,
                price: *price,
                quantity: *quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                extra_fields: (),
            },
            OrderType::IcebergOrder {
                id,
                price,
                visible_quantity,
                hidden_quantity,
                side,
                timestamp,
                time_in_force,
                ..
            } => OrderType::IcebergOrder {
                id: *id,
                price: *price,
                visible_quantity: *visible_quantity,
                hidden_quantity: *hidden_quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                extra_fields: (),
            },
            OrderType::PostOnly {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
                ..
            } => OrderType::PostOnly {
                id: *id,
                price: *price,
                quantity: *quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                extra_fields: (),
            },
            OrderType::TrailingStop {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
                trail_amount,
                last_reference_price,
                ..
            } => OrderType::TrailingStop {
                id: *id,
                price: *price,
                quantity: *quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                trail_amount: *trail_amount,
                last_reference_price: *last_reference_price,
                extra_fields: (),
            },
            OrderType::PeggedOrder {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
                reference_price_offset,
                reference_price_type,
                ..
            } => OrderType::PeggedOrder {
                id: *id,
                price: *price,
                quantity: *quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                reference_price_offset: *reference_price_offset,
                reference_price_type: *reference_price_type,
                extra_fields: (),
            },
            OrderType::MarketToLimit {
                id,
                price,
                quantity,
                side,
                timestamp,
                time_in_force,
                ..
            } => OrderType::MarketToLimit {
                id: *id,
                price: *price,
                quantity: *quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                extra_fields: (),
            },
            OrderType::ReserveOrder {
                id,
                price,
                visible_quantity,
                hidden_quantity,
                side,
                timestamp,
                time_in_force,
                replenish_threshold,
                replenish_amount,
                auto_replenish,
                ..
            } => OrderType::ReserveOrder {
                id: *id,
                price: *price,
                visible_quantity: *visible_quantity,
                hidden_quantity: *hidden_quantity,
                side: *side,
                timestamp: *timestamp,
                time_in_force: *time_in_force,
                replenish_threshold: *replenish_threshold,
                replenish_amount: *replenish_amount,
                auto_replenish: *auto_replenish,
                extra_fields: (),
            },
        }
    }
}
