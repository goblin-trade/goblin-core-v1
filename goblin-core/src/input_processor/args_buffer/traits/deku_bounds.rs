use deku::DekuReader;
#[cfg(feature = "encode")]
use deku::DekuWriter;

/// Everything a type must implement to be read from the wire and, on `encode`
/// builds, written back to it.
///
/// deku's `bits` feature is disabled, so there is no bit-sized context: bit
/// fields are decoded by hand from whole-byte lanes (see
/// [`bit_lane`](crate::input_processor::bit_lane)), and the default (unit)
/// context is the only one required.
///
/// This exists so `QuantityOps` can require `DekuWriter` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait DekuBounds: for<'a> DekuReader<'a> + DekuWriter {}
#[cfg(feature = "encode")]
impl<T> DekuBounds for T where T: for<'a> DekuReader<'a> + DekuWriter {}

/// Decode-only variant used when the `encode` feature is disabled.
#[cfg(not(feature = "encode"))]
pub trait DekuBounds: for<'a> DekuReader<'a> {}
#[cfg(not(feature = "encode"))]
impl<T: for<'a> DekuReader<'a>> DekuBounds for T {}
