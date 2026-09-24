use crate::codec::GoblinRead;
#[cfg(feature = "encode")]
use crate::codec::GoblinWrite;

/// Everything a type must implement to be read from the wire and, on `encode`
/// builds, written back to it.
///
/// The default (unit) context is the only one required here. Types that need
/// out-of-band decoding parameters take them as a `Ctx` on the trait instead.
///
/// This exists so `QuantityOps` can require `GoblinWrite` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait CodecBounds: for<'de> GoblinRead<'de> + GoblinWrite {}
#[cfg(feature = "encode")]
impl<T> CodecBounds for T where T: for<'de> GoblinRead<'de> + GoblinWrite {}

/// Decode-only variant used when the `encode` feature is disabled.
#[cfg(not(feature = "encode"))]
pub trait CodecBounds: for<'de> GoblinRead<'de> {}
#[cfg(not(feature = "encode"))]
impl<T: for<'de> GoblinRead<'de>> CodecBounds for T {}
