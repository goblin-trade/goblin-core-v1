use deku::DekuReader;
#[cfg(feature = "encode")]
use deku::DekuWriter;
use deku::ctx::{BitSize, Order};

/// Everything a type must implement to be read from the wire and, on `encode`
/// builds, written back to it.
///
/// Besides the default-context reader/writer, this requires the bit-sized
/// context `(BitSize, Order)`. That lets bit-packed headers declare a quantity
/// field directly (e.g. `#[deku(bits = "30")] num_lots: U32Variant<..>`) instead
/// of routing the wire value through a `map` closure.
///
/// This exists so `QuantityOps` can require `DekuWriter` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait DekuBounds:
    for<'a> DekuReader<'a>
    + for<'a> DekuReader<'a, (BitSize, Order)>
    + DekuWriter
    + DekuWriter<(BitSize, Order)>
{
}
#[cfg(feature = "encode")]
impl<T> DekuBounds for T where
    T: for<'a> DekuReader<'a>
        + for<'a> DekuReader<'a, (BitSize, Order)>
        + DekuWriter
        + DekuWriter<(BitSize, Order)>
{
}

/// Decode-only variant used when the `encode` feature is disabled.
#[cfg(not(feature = "encode"))]
pub trait DekuBounds: for<'a> DekuReader<'a> + for<'a> DekuReader<'a, (BitSize, Order)> {}
#[cfg(not(feature = "encode"))]
impl<T: for<'a> DekuReader<'a> + for<'a> DekuReader<'a, (BitSize, Order)>> DekuBounds for T {}
