use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod const_default;
mod fixed_codec;
mod fixed_decode;

/// Derive `FixedDecode` for a fixed-size struct whose fields all implement
/// `FixedDecode`.
///
/// - `ENCODED_SIZE` is generated as the sum of each field's `ENCODED_SIZE`.
/// - `decode_raw` decodes fields in declaration order, so each field reads
///   from wherever the previous field left the cursor. This matches how
///   `Areader`'s offset is threaded through `decode_raw` calls today.
///
/// Only structs with named fields are supported (no tuple structs, no unit
/// structs, no enums) since we're only targeting fixed-size, heap-free wire
/// structs.
///
/// ```ignore
/// #[derive(FixedDecode)]
/// struct Header {
///     kind: u8,
///     len: u16,
///     flags: u32,
/// }
/// ```
#[proc_macro_derive(FixedDecode)]
pub fn derive_decodable_v2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match fixed_decode::expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive `FixedCodec` for a fixed-size struct whose fields are byte-aligned
/// `FixedCodec` implementors and/or explicitly-sized sub-byte bit fields.
///
/// This is the encode/decode-expanded counterpart of the `FixedDecode` derive:
/// the generated `impl` provides both `raw_fixed_decode` and `raw_fixed_encode`.
///
/// # Bit-level syntax
///
/// A field annotated with `#[codec(bits = N)]` is packed into a sub-byte slot
/// of `N` bits, LSB-first, matching the hand-written decoders in this crate
/// (so the first field is the least-significant bit). Sub-byte fields are only
/// used when such a width is explicitly requested; unannotated fields stay
/// byte-aligned. Consecutive sub-byte fields are gathered into the smallest
/// integer lane (`u8`/`u16`/`u32`/`u64`) that fits them.
///
/// ```ignore
/// #[derive(FixedCodec)]
/// struct HeaderFlags {
///     #[codec(bits = 1)] read_custom_recipient: bool,
///     #[codec(bits = 1)] read_msg_value: bool,
///     #[codec(bits = 1)] process_dynamic_markets: bool,
///     #[codec(bits = 1)] withdraw_eth: bool,
///     #[codec(bits = 1)] withdraw_internally: bool,
///     #[codec(bits = 3)] custom_erc20_count: usize,
/// }
/// ```
#[proc_macro_derive(FixedCodec, attributes(codec))]
pub fn derive_fixed_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match fixed_codec::expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive `ConstDefault` for a struct whose fields all implement `ConstDefault`
/// (including fixed-size array fields like `[T; N]`, assuming `ConstDefault`
/// has a blanket impl for arrays).
///
/// `ZEROED` is built by calling `ZEROED` on every field, in declaration
/// order. Supports both named and tuple structs; unit structs, enums, and
/// unions are rejected at compile time.
#[proc_macro_derive(ConstDefault)]
pub fn derive_const_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match const_default::expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
