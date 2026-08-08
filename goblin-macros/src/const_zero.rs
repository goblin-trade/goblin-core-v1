use quote::format_ident;
use quote::quote;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    // Stub markers (`struct ETHStub;`) are unit structs with no fields at
    // all: there's nothing to zero, so `ZEROED` is just `Self`. This is
    // tracked separately from `is_tuple` since a unit struct's constructor
    // is the bare path `Self`, not `Self { .. }` or `Self(..)`.
    let is_unit;
    let (field_names, field_types, is_tuple): (Vec<_>, Vec<_>, bool) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                is_unit = false;
                let names = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().expect("named field"))
                    .collect();
                let types = fields.named.iter().map(|f| f.ty.clone()).collect();
                (names, types, false)
            }
            Fields::Unnamed(fields) => {
                is_unit = false;
                // Tuple structs have no field idents. We only need
                // placeholder identifiers here to keep the same shape as
                // the named-field case; the actual constructor for tuple
                // structs is positional (`Self(...)`) and doesn't use
                // these names.
                let names = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field_{}", i))
                    .collect();
                let types = fields.unnamed.iter().map(|f| f.ty.clone()).collect();
                (names, types, true)
            }
            Fields::Unit => {
                is_unit = true;
                (Vec::new(), Vec::new(), false)
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span(),
                "ConstZero cannot be derived for enums",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "ConstZero cannot be derived for unions",
            ));
        }
    };

    // ConstZero carries no lifetime of its own (unlike FixedDecode), so the
    // struct's own generics can be used as-is for both the impl and Self.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Fully-qualified path to the trait, so callers never need to import it.
    let trait_path = quote! { crate::settlement::traits::ConstZero };

    let zero_exprs = field_types
        .iter()
        .map(|ty| quote! { <#ty as #trait_path>::ZEROED });

    // Named structs build `Self { a: <Ty as ConstZero>::ZEROED, ... }`;
    // tuple structs build `Self(<Ty as ConstZero>::ZEROED, ...)` — field
    // order matches declaration order in both cases, and fixed-size array
    // fields (e.g. `[u8; 32]`) need no special handling: they're just
    // another field type `T` for which `<T as ConstZero>::ZEROED` resolves
    // via a blanket `impl<T: ConstZero, const N: usize> ConstZero for
    // [T; N]` defined alongside the trait itself.
    let constructor = if is_unit {
        quote! { Self }
    } else if is_tuple {
        quote! { Self(#(#zero_exprs),*) }
    } else {
        quote! { Self { #(#field_names: #zero_exprs),* } }
    };

    let expanded = quote! {
        impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            const ZEROED: Self = #constructor;
        }
    };

    Ok(expanded)
}
