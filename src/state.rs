use scrypto::prelude::*;

#[derive(ScryptoSbor)]
pub enum Side {
    Long,
    Short,
}

#[derive(ScryptoSbor)]
pub struct Position {
    pub side: Side,
    pub leverage: Decimal,
    pub open_price: Decimal,
    pub borrowed_size: Decimal,
    pub collateral_size: Decimal,
}

impl Position {
    pub fn new(
        side: Side,
        leverage: Decimal,
        open_price: Decimal,
        borrowed_size: Decimal,
        collateral_size: Decimal,
    ) -> Self {
        Self {
            side,
            leverage,
            open_price,
            borrowed_size,
            collateral_size,
        }
    }
}