use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod const_default;
mod fixed_codec;

/// Generate `FixedCodec` (encode + decode) for a fixed-size struct.
///
/// This is the single fixed-size codec for the wire: the generated `impl`
/// provides both `raw_fixed_decode` and `raw_fixed_encode`.
/// It is applied as an attribute so the packed size can be passed directly.
///
/// ```ignore
/// #[fixed_codec(bits = 40)]
/// struct MakeHeader {
///     inner_pos: InnerPos,           // full 8 bits
///     occupancy_enum: OccupancyEnum, // 1 bit
///     inner_enum_raw: bool,          // 1 bit
///     base_lots_u32: BaseLots<u32>,  // remaining 30 bits
/// }
/// ```
///
/// # Width inference
///
/// With a top-level `bits = N`, the struct is one little-endian bit stream of
/// `N` bits (padded to a byte) and widths are inferred:
///
/// * a field's explicit `#[codec(bits = M)]` wins;
/// * otherwise a field takes its type's `BitPack::CAPACITY` — 1 for `bool`, the
///   full width for a newtype over `u8`/`u16`/..., 1 for a generated axis enum.
///   Numbers that appear *before* the flags therefore occupy their full width;
/// * the **last** field without an explicit width absorbs the remainder
///   `N - (sum of the other widths)` — the number after the bools/enum.
///
/// Without `bits`, fields carrying `#[codec(bits = M)]` are gathered into
/// integer lanes and unannotated fields stay byte-aligned. `bool` is always
/// one bit and rejects any other width.
#[proc_macro_attribute]
pub fn fixed_codec(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as fixed_codec::BitsArgs);
    let item = parse_macro_input!(item as syn::ItemStruct);

    match fixed_codec::expand_attribute(item, args.bits, args.validate) {
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
