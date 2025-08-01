// #[repr(C, packed)]
// #[derive(Clone, Copy)]
// pub struct Side(bool);

#[repr(u8)]
#[derive(PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

impl From<bool> for Side {
    fn from(value: bool) -> Self {
        match value {
            true => Side::Bid,
            false => Side::Ask,
        }
    }
}
