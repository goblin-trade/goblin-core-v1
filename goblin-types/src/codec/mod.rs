//! Read/write traits for Goblin wire formats.
//!
//! These replace the `deku`-based decoding that `goblin-core` used to rely on.
//! The trait is deliberately **not** generic over the backing reader: decoding
//! happens directly over a borrowed `&[u8]`, so a borrowing decoder (a zero-copy
//! token list, say) can name the backing slice and hand back `&'de` references
//! without threading a separate "source" through a context.
//!
//! Sub-byte fields are not modelled here: the wire format is byte-oriented and
//! bit packing is decoded by hand out of whole-byte lanes.
//!
//! Endianness is explicit and little-endian. (deku defaulted to the target's
//! native endianness, which is little-endian on every target this crate builds
//! for; pinning it here keeps the wire format portable and unambiguous.)
//!
//! The module is split so each concern lives on its own:
//! - `error`: the [`CodecError`] type and [`CodecResult`] alias.
//! - `reader`: the zero-copy [`Reader`].
//! - `writer`: the [`Writer`].
//! - `traits`: the [`GoblinRead`]/[`GoblinWrite`] traits.
//! - `impls`: the built-in integer and byte-array implementations.

mod error;
mod impls;
mod reader;
mod traits;
mod writer;

pub use error::{CodecError, CodecResult};
pub use reader::Reader;
pub use traits::{GoblinRead, GoblinWrite};
pub use writer::Writer;
