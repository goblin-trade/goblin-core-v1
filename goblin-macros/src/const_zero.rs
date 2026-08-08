use quote::format_ident;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, spanned::Spanned};

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

    // For a plain field type `T`, zeroing is `<T as ConstZero>::ZEROED`.
    // For a fixed-size array field `[T; N]`, we do NOT require `ConstZero`
    // to be implemented on the array type itself (there's no blanket
    // `impl<T: ConstZero, const N: usize> ConstZero for [T; N]` in this
    // codebase, only ad-hoc impls like `[u8; 20]` for `Address`). Instead
    // we recurse into the element type and build the array-repeat
    // expression directly, e.g. `[<T as ConstZero>::ZEROED; N]` — this is
    // exactly what the hand-written impls do, and it also handles nested
    // arrays (`[[T; N]; M]`) for free via recursion.
    fn zero_expr(ty: &Type, trait_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        if let Type::Array(array) = ty {
            let elem_expr = zero_expr(&array.elem, trait_path);
            let len = &array.len;
            quote! { [#elem_expr; #len] }
        } else {
            quote! { <#ty as #trait_path>::ZEROED }
        }
    }

    let zero_exprs = field_types.iter().map(|ty| zero_expr(ty, &trait_path));

    // Named structs build `Self { a: <Ty as ConstZero>::ZEROED, ... }`;
    // tuple structs build `Self(<Ty as ConstZero>::ZEROED, ...)` — field
    // order matches declaration order in both cases. Array fields use the
    // repeat-expression form produced by `zero_expr` above instead of
    // requiring `ConstZero` on the array type.
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
