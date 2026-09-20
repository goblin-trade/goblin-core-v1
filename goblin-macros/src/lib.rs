use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod const_default;

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
