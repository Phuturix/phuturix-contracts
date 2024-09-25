use scrypto::prelude::*;

#[derive(ScryptoSbor, Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Long,
    Short,
}

#[derive(ScryptoSbor, Debug, Clone, PartialEq, Eq)]
pub enum PositionState {
    Open,
    Closed,
}

#[derive(ScryptoSbor, Debug, Clone)]
pub struct Position {
    pub state: PositionState,
    pub side: Side,
    pub leverage: Decimal,
    pub open_price: Decimal,
    pub order_quantity: Decimal,
    //pub collateral_size: Decimal,
}

impl Position {
    pub fn new(
        state: PositionState,
        side: Side,
        leverage: Decimal,
        open_price: Decimal,
        order_quantity: Decimal,
        //collateral_size: Decimal,
    ) -> Self {
        Self {
            state,
            side,
            leverage,
            open_price,
            order_quantity,
            //collateral_size,
        }
    }

    pub fn calculate_initial_margin(&self) -> Decimal {
        (self.open_price * self.order_quantity) / self.leverage
    }

    pub fn calculate_fee_to_open_position(&self, fee_rate: Decimal) -> Decimal {
        self.open_price * self.order_quantity * fee_rate
    }

    pub fn calculate_liquidation_price(&self) -> Decimal {
        match self.side {
            Side::Long => (self.open_price * (self.leverage - Decimal::one())) / self.leverage,
            Side::Short => (self.open_price * (self.leverage + Decimal::one())) / self.leverage,
        }
    }

    pub fn calculate_fee_to_close_position(&self, fee_rate: Decimal) -> Decimal {
        self.order_quantity * self.calculate_liquidation_price() * fee_rate
    }

    pub fn calculate_total_cost(&self, fee_rate: Decimal) -> Decimal {
        self.calculate_initial_margin() +
        self.calculate_fee_to_open_position(fee_rate) +
        self.calculate_fee_to_close_position(fee_rate)
    }

}
