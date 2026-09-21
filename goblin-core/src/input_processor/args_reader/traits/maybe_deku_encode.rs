/// Encoder side of a quantity, required only when the `encode` feature is on.
///
/// This exists so [`QuantityOps`] can require `DekuWriter` on encode builds
/// without a `where`-clause attribute (an unstable feature that rust-analyzer
/// cannot parse).
#[cfg(feature = "encode")]
pub trait MaybeDekuEncode: DekuWriter {}
#[cfg(feature = "encode")]
impl<T: DekuWriter> MaybeDekuEncode for T {}

/// Encoder side of a quantity; a no-op when `encode` is disabled.
#[cfg(not(feature = "encode"))]
pub trait MaybeDekuEncode {}
#[cfg(not(feature = "encode"))]
impl<T> MaybeDekuEncode for T {}
