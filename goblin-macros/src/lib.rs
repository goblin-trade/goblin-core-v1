use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod const_default;
mod fixed_decode;

/// Derive `FixedDecode` for a fixed-size struct whose fields all implement
/// `FixedDecode`.
///
/// - `ENCODED_SIZE` is generated as the sum of each field's `ENCODED_SIZE`.
/// - `decode_raw` decodes fields in declaration order, so each field reads
///   from wherever the previous field left the cursor. This matches how
///   `DecodeCtx`'s offset is threaded through `decode_raw` calls today.
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
