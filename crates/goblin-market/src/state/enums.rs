use crate::quantities::Ticks;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

impl From<bool> for Side {
    fn from(value: bool) -> Side {
        match value {
            true => Side::Bid,
            false => Side::Ask,
        }
    }
}

impl Side {
    pub fn opposite(&self) -> Self {
        match *self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }

    /// Find the side of an order ID during removals
    ///
    /// # Notes
    ///
    /// * Since no orders are present between best bid price and best ask price,
    /// return None if price falls within the spread.
    ///
    /// * Additionally we can ensure that orders are sorted in correct order
    /// moving away from the centre by using the last price instead of best
    /// market price to determine side.
    pub fn from_removal_price(
        price: Ticks,
        last_bid_price: Ticks,
        last_ask_price: Ticks,
    ) -> Option<Self> {
        debug_assert!(last_ask_price > last_bid_price);

        if price >= last_ask_price {
            Some(Side::Ask)
        } else if price <= last_bid_price {
            Some(Side::Bid)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub enum SelfTradeBehavior {
    Abort,
    CancelProvide,
    DecrementTake,
}

impl From<u8> for SelfTradeBehavior {
    fn from(value: u8) -> SelfTradeBehavior {
        match value {
            0 => SelfTradeBehavior::Abort,
            1 => SelfTradeBehavior::CancelProvide,
            2 => SelfTradeBehavior::DecrementTake,
            _ => panic!("Invalid value for SelfTradeBehavior"), // Handle invalid inputs
        }
    }
}
