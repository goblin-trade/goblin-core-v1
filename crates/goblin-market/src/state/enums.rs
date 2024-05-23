#[derive(PartialEq, Clone)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    pub fn init(is_bid: bool) -> Self {
        match is_bid {
            true => Side::Bid,
            false => Self::Ask,
        }
    }

    pub fn opposite(&self) -> Self {
        match *self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
}
