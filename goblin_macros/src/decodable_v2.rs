use quote::quote;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new(
                    data.fields.span(),
                    "DecodableV2 can only be derived for structs with named fields, not tuple structs",
                ));
            }
            Fields::Unit => {
                return Err(syn::Error::new(
                    data.fields.span(),
                    "DecodableV2 can only be derived for structs with at least one field",
                ));
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span(),
                "DecodableV2 cannot be derived for enums",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "DecodableV2 cannot be derived for unions",
            ));
        }
    };

    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.clone().expect("named field"))
        .collect();
    let field_types: Vec<_> = fields.iter().map(|f| f.ty.clone()).collect();

    let size_terms = field_types
        .iter()
        .map(|ty| quote! { <#ty as DecodableV2>::ENCODED_SIZE });

    // Decoded in declaration order: each `decode_raw` call advances the
    // shared `ctx` offset, so the next field naturally starts where the
    // previous one left off.
    let decode_stmts = field_names
        .iter()
        .zip(field_types.iter())
        .map(|(field, ty)| {
            quote! {
                let #field = <#ty as DecodableV2>::decode_raw(ctx);
            }
        });

    let expanded = quote! {
        impl #impl_generics DecodableV2 for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #size_terms)*;

            fn decode_raw(ctx: &DecodeCtx) -> Self {
                #(#decode_stmts)*
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    Ok(expanded)
}
