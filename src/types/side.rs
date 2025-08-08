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
