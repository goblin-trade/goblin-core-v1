pub struct BitsLayout<const BITS: u16>;

impl<const BITS: u16> BitsLayout<BITS> {
    pub const OFFSET: u16 = BITS >> 8;
    pub const BIT_COUNT: u16 = BITS & 0b1111;

    /// Unshifted bitmask of BIT_COUNT ones
    /// e.g. BIT_COUNT=4 → (0b10000 - 1) = 0b1111
    ///
    /// Special case: 1u64 << 64 will overflow for BIT_COUNT = 64.
    /// Use result directly. The code will be branchless as we
    /// are dealing with consts.
    pub const MASK: u64 = if Self::BIT_COUNT == 64 {
        u64::MAX
    } else {
        (1u64 << Self::BIT_COUNT) - 1
    };

    pub const MAX: u64 = Self::MASK;

    pub const fn step_interval() -> usize {
        (1u64 << Self::OFFSET) as usize
    }
}
