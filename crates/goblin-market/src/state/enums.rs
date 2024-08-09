#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    Bid,
    Ask,
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
    // pub fn init(is_bid: bool) -> Self {
    //     match is_bid {
    //         true => Side::Bid,
    //         false => Self::Ask,
    //     }
    // }

    pub fn opposite(&self) -> Self {
        match *self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
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
