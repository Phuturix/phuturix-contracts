use scrypto::prelude::*;

use phuturix::state::{Position, Side, PositionState};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_position(side: Side) -> Position {
        Position::new( 
            PositionState::Open,
            side,
            dec!("5"),
            dec!("100"),
            dec!("10"),
        )
    }

    #[test]
    fn test_new_position() {
        let position = create_test_position(Side::Long);
        assert_eq!(position.state, PositionState::Open);
        assert_eq!(position.side, Side::Long);
        assert_eq!(position.leverage, dec!("5"));
        assert_eq!(position.open_price, dec!("100"));
        assert_eq!(position.order_quantity, dec!("10"));
    }

    #[test]
    fn test_calculate_initial_margin() {
        let position = create_test_position(Side::Long);
        assert_eq!(position.calculate_initial_margin(), dec!("200"));
    }

    #[test]
    fn test_calculate_fee_to_open_position() {
        let position = create_test_position(Side::Long);
        let fee_rate = dec!("0.001");
        assert_eq!(position.calculate_fee_to_open_position(fee_rate), dec!("1"));
    }

    #[test]
    fn test_calculate_liquidation_price() {
        let long_position = create_test_position(Side::Long);
        assert_eq!(long_position.calculate_liquidation_price(), dec!("80"));

        let short_position = create_test_position(Side::Short);
        assert_eq!(short_position.calculate_liquidation_price(), dec!("120"));
    }

    #[test]
    fn test_calculate_fee_to_close_position() {
        let position = create_test_position(Side::Long);
        let fee_rate = dec!("0.001");
        assert_eq!(position.calculate_fee_to_close_position(fee_rate), dec!("0.8"));
    }

    #[test]
    fn test_calculate_total_cost() {
        let position = create_test_position(Side::Long);
        let fee_rate = dec!("0.001");
        let expected_total_cost = dec!("200") + dec!("1") + dec!("0.8");
        assert_eq!(position.calculate_total_cost(fee_rate), expected_total_cost);
    }
}