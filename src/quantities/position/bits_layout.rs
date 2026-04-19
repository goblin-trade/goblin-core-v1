pub struct BitsLayout<const BITS: u16>;

impl<const BITS: u16> BitsLayout<BITS> {
    pub const OFFSET: u16 = BITS >> 8;
    pub const COUNT: u16 = BITS & 0b1111;

    /// Unshifted bitmask of BIT_COUNT ones
    /// e.g. BIT_COUNT=4 → 0b1111
    /// Note: not legal for BIT_COUNT = 64
    pub const MASK: u64 = (1u64 << Self::COUNT) - 1;

    pub const MAX: u64 = Self::MASK;

    pub fn step_interval() -> usize {
        (1u64 << Self::OFFSET) as usize
    }
}
