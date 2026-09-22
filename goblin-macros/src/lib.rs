use proc_macro::TokenStream;
use syn::{DeriveInput, ItemEnum, parse_macro_input};

mod const_default;
mod define_axis;

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

/// Generate the plumbing for a trading axis enum.
///
/// Applies to an enum whose variants carry explicit (or implicit) integer
/// discriminants, and emits:
///
/// * a seed struct named after the enum with the trailing `Enum` stripped
///   (override with `#[define_axis(seed = Name)]`), used only to parameterise
///   `Marker`,
/// * `From<bool>` (binary axes only), `TryFrom<u8>`, and `from_raw`,
/// * a `Marker` type alias plus an `AxisMarker` impl for each variant.
///
/// The enum keeps its own attributes verbatim, so derives that only some axes
/// need (such as Deku) can be added per-enum rather than to every axis. To
/// have those attributes forwarded, place them *after* `#[define_axis]`.
#[proc_macro_attribute]
pub fn define_axis(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemEnum);

    match define_axis::expand(attr.into(), item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
