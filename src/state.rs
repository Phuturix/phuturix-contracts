use scrypto::prelude::*;

#[derive(ScryptoSbor, Debug)]
pub enum Side {
    Long,
    Short,
}

#[derive(ScryptoSbor, Debug)]
pub enum PositionState {
    Open,
    Closed,
}

#[derive(ScryptoSbor, Debug)]
pub struct Position {
    pub state: PositionState,
    pub side: Side,
    pub leverage: Decimal,
    pub open_price: Decimal,
    pub borrowed_size: Decimal,
    pub collateral_size: Decimal,
}

impl Position {
    pub fn new(
        state: PositionState,
        side: Side,
        leverage: Decimal,
        open_price: Decimal,
        borrowed_size: Decimal,
        collateral_size: Decimal,
    ) -> Self {
        Self {
            state,
            side,
            leverage,
            open_price,
            borrowed_size,
            collateral_size,
        }
    }
}
