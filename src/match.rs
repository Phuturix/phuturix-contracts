use scrypto::prelude::*;
use create::state::*;
pub fn calculate_initial_margin(oder_price: f32, order_quantity: f32, leverage: f32) -> f32{
    (oder_price * order_quantity) / leverage
}

pub fn calculate_fee_to_open_position(oder_price: f32, order_quantity: f32, fee: f32) -> f32{
    oder_price * order_quantity * fee
}

pub fn calculate_liquidation_price(oder_price: f32, leverage: f32, side: Side) -> f32 {
    if side== Side::Long {
        (oder_price * (leverage + 1)) / leverage
    } else {
        (oder_price * (leverage - 1)) / leverage
    }

}

pub fn calculate_fee_to_close_position(order_quantity: f32, liquidation_price: f32, fee: f32) -> f32 {
    order_quantity * liquidation_price * fee
}

pub fn calculate_total_order_cost(initial_margin: f32, fee_to_open: f32, fee_to_close: f32) -> f32 {
    initial_margin + fee_to_open + fee_to_close
}

pub fn calculate_order_quantity(order_cost: f32, leverage: f32, order_price: f32, fee: f32, side: Side) -> f32 {
    let factor = if side == Side::Long {
        0.9996
    } else {
        1.0004
    };
    (order_cost * leverage) / (order_price * (0.0008 * leverage + factor))
}