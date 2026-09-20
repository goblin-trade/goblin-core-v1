//! Zero-copy decode: borrow values straight out of a reader's buffer.
//!
//! [`ZeroCopyReadV2`] works with deku's [`Reader`], exposing unchecked
//! reinterpretation of the reader's backing `&'a [u8]`.
//!
//! [`Reader`]: deku::reader::Reader

mod zero_copy_read_v2;

pub use zero_copy_read_v2::*;
