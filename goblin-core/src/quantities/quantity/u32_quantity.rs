use crate::input_processor::BitPack;
use crate::quantities::{Exp, Quantity, QuantityOps};

/// Associates a 64 bit quantity with its compact 32 bit variant.
///
/// 32 bit variants are used in wire headers to reduce encoded size. They are
/// widened back to the 64 bit quantity before internal math.
pub trait U32Quantity: Sized {
    /// Equivalent quantity backed by a `u32`, decodable from the wire.
    type U32Variant: QuantityOps + From<u32> + Into<Self> + BitPack;
}

/// Convenience alias for the 32 bit variant of a quantity.
///
/// `Q::U32Variant` cannot be written directly (chained associated type paths are
/// ambiguous), so use `U32Variant<Q>` instead.
pub type U32Variant<Q> = <Q as U32Quantity>::U32Variant;

impl<E> U32Quantity for Quantity<E, u64>
where
    E: Exp,
{
    type U32Variant = Quantity<E, u32>;
}
