use deku::DekuReader;
#[cfg(feature = "encode")]
use deku::DekuWriter;

/// Everything a type must implement to be read from the wire and, on `encode`
/// builds, written back to it.
///
/// This exists so `QuantityOps` can require `DekuWriter` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait DekuBounds: for<'a> DekuReader<'a> + DekuWriter {}
#[cfg(feature = "encode")]
impl<T: for<'a> DekuReader<'a> + DekuWriter> DekuBounds for T {}

/// Decode-only variant used when the `encode` feature is disabled.
#[cfg(not(feature = "encode"))]
pub trait DekuBounds: for<'a> DekuReader<'a> {}
#[cfg(not(feature = "encode"))]
impl<T: for<'a> DekuReader<'a>> DekuBounds for T {}
