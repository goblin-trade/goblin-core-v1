// #[repr(C, packed)]
// #[derive(Clone, Copy)]
// pub struct Side(bool);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}
