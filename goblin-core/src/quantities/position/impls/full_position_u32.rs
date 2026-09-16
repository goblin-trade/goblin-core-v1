use crate::quantities::FullPosU32;

/// Scaled down version of FullPosition used in decoding
pub trait FullPositionU32 {
    const ZERO: Self;
    const MAX: Self;
}

impl FullPositionU32 for FullPosU32 {
    const ZERO: Self = Self::new(u32::MIN);
    const MAX: Self = Self::new(u32::MAX);
}
