mod read;
#[cfg(feature = "encode")]
mod write;

/// Shared codec context for [`CustomERC20List`](super::CustomERC20List), used by
/// both the reader and writer.
///
/// Carries the number of tokens to decode. The backing slice for the zero-copy
/// borrow is taken straight off the codec reader, so it is no longer carried
/// here.
#[derive(Clone, Copy)]
pub struct CustomERC20ListCtx {
    pub count: usize,
}
