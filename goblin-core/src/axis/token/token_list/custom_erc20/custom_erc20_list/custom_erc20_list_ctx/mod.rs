mod read;
#[cfg(feature = "encode")]
mod write;

/// Shared Deku context for [`CustomERC20List`](super::CustomERC20List), used by
/// both the reader and writer.
///
/// Zero-copy borrowing needs the reader's backing `&[u8]`, but a
/// [`DekuReader`](deku::DekuReader) impl is generic over the reader `R` and
/// cannot recover that slice, so it is threaded through the context alongside
/// the token count. The zero-copy read and the flat write live in the sibling
/// `read` and `write` modules, so the derived impls only name them.
#[derive(Clone, Copy)]
pub struct CustomERC20ListCtx<'a> {
    pub count: usize,
    pub source: &'a [u8],
}
