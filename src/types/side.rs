use crate::quantities::{
    AdjustedQuoteLots, BaseLots, QuoteLots, QuoteLotsPerBaseUnitPerTick, Ticks,
};

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

impl From<bool> for Side {
    #[inline]
    fn from(value: bool) -> Self {
        // SAFETY: bool is guaranteed to be 0 (false) or 1 (true),
        // which directly maps to our enum discriminants
        unsafe { core::mem::transmute(value) }
    }
}

impl From<Side> for bool {
    #[inline]
    fn from(value: Side) -> bool {
        // SAFETY: Side enum has discriminants 0 and 1, which are valid bool values
        unsafe { core::mem::transmute(value as u8) }
    }
}

impl Side {
    /// Returns the opposite side in a branchless manner.
    /// Bid becomes Ask, Ask becomes Bid.
    #[inline]
    pub const fn opposite(self) -> Self {
        // SAFETY: XOR with 1 flips bit 0: 0 becomes 1, 1 becomes 0
        // This directly maps to our enum discriminants (Bid=0, Ask=1)
        unsafe { core::mem::transmute((self as u8) ^ 1) }
    }
}

pub struct Bid;
pub struct Ask;

pub trait SideMarker {
    type Lots;
    type Quote;
    type Opposite;

    const INDEX: usize;
    const SIDE: Side;

    // fn get_quote(
    //     size: BaseLots,
    //     tick_size: QuoteLotsPerBaseUnitPerTick,
    //     price: Ticks,
    // ) -> Self::Quote;
}

impl SideMarker for Bid {
    type Lots = QuoteLots;
    type Quote = AdjustedQuoteLots;
    type Opposite = Ask;

    const INDEX: usize = 0;
    const SIDE: Side = Side::Bid;
}

impl Bid {
    pub fn get_quote(
        size: BaseLots,
        tick_size: QuoteLotsPerBaseUnitPerTick,
        price: Ticks,
    ) -> <Bid as SideMarker>::Quote {
        (tick_size * price) * size
    }
}

impl SideMarker for Ask {
    type Lots = BaseLots;
    type Quote = BaseLots;
    type Opposite = Bid;

    const INDEX: usize = 1;
    const SIDE: Side = Side::Ask;

    // fn get_quote(
    //     size: BaseLots,
    //     _tick_size: QuoteLotsPerBaseUnitPerTick,
    //     _price: Ticks,
    // ) -> Self::Quote {
    //     size
    // }
}

impl Ask {
    pub fn get_quote(size: BaseLots) -> <Ask as SideMarker>::Quote {
        size
    }
}
