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
    /// Since removals cannot happen within the spread, we can ignore prices
    /// within the spread
    pub fn from_removal_price(
        price: Ticks,
        best_bid_price: Ticks,
        best_ask_price: Ticks,
    ) -> Option<Self> {
        debug_assert!(best_ask_price > best_bid_price);

        if price >= best_ask_price {
            Some(Side::Ask)
        } else if price <= best_bid_price {
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
