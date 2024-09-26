use scrypto::prelude::*;
use crate::state::{Position, PositionState, Side};

pub fn close_position(
    position: &mut Position,
    current_price: Decimal,
    fee: Decimal,
    pool: &mut Vault
) -> (Decimal, Decimal) {
    // Calculate the price difference
    let price_difference = calculate_price_difference(position, current_price);
    
    // Calculate the fee for closing the position
    let close_fee = calculate_fee_to_close_position(position.order_quantity, current_price, fee);
    
    // Calculate the total return (price difference minus closing fee)
    let total_return = price_difference - close_fee;
    
    // Update the pool's funds
    if total_return > Decimal::zero() {
        // If profitable, pay from the pool
        let payment = pool.take(total_return);
        // TODO: Transfer payment to the user
    } else if total_return < Decimal::zero() {
        // If loss, collect from the user and deposit to the pool
        // TODO: Collect -total_return from the user and deposit into the pool
    }
    
    // Update position state
    position.state = PositionState::Closed;
    
    (price_difference, close_fee)
}

pub fn calculate_price_difference(position: &Position, current_price: Decimal) -> Decimal {
    let price_difference = match position.side {
        Side::Long => current_price - position.open_price,
        Side::Short => position.open_price - current_price,
    };
    
    price_difference * position.order_quantity * position.leverage
}

pub fn calculate_initial_margin(open_price: Decimal, order_quantity: Decimal, leverage: Decimal) -> Decimal {
    (open_price * order_quantity) / leverage
}

pub fn calculate_fee_to_open_position(open_price: Decimal, order_quantity: Decimal, fee: Decimal) -> Decimal {
    open_price * order_quantity * fee
}

pub fn calculate_liquidation_price(open_price: Decimal, leverage: Decimal, side: Side) -> Decimal {
    match side {
        Side::Long => (open_price * (leverage + Decimal::one())) / leverage,
        Side::Short => (open_price * (leverage - Decimal::one())) / leverage,
    }
}

pub fn calculate_fee_to_close_position(order_quantity: Decimal, liquidation_price: Decimal, fee: Decimal) -> Decimal {
    order_quantity * liquidation_price * fee
}

pub fn calculate_total_order_cost(initial_margin: Decimal, fee_to_open: Decimal, fee_to_close: Decimal) -> Decimal {
    initial_margin + fee_to_open + fee_to_close
}