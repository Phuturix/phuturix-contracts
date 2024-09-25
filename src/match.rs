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

