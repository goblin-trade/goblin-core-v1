mod impl_variable_decode;

use crate::{
    axis::leg::LegMatcher,
    quantities::{FullPosU32, U32Variant},
};

pub struct TakeHeaderOptional<In: LegMatcher> {
    /// The minimum number of base lots to fill, otherwise the order will be invalidated.
    pub min_lots_to_fill_u32: U32Variant<In::Lots>,

    /// The worst position to be matched against. Stop matching after this price is crossed.
    pub limit_u32: FullPosU32,
}
