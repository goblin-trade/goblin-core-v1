use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod decodable_v2;

/// Derive `DecodableV2` for a fixed-size struct whose fields all implement
/// `DecodableV2`.
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
/// #[derive(DecodableV2)]
/// struct Header {
///     kind: u8,
///     len: u16,
///     flags: u32,
/// }
/// ```
#[proc_macro_derive(DecodableV2)]
pub fn derive_decodable_v2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match decodable_v2::expand(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
