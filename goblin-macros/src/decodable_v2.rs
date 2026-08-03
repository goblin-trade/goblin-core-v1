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

    // Fully-qualified path to the trait, so callers never need to import it.
    let trait_path = quote! { crate::input_processor::DecodableV2 };
    let ctx_path = quote! { crate::input_processor::DecodeCtx };

    let size_terms = field_types
        .iter()
        .map(|ty| quote! { <#ty as #trait_path>::ENCODED_SIZE });

    let decode_stmts = field_names
        .iter()
        .zip(field_types.iter())
        .map(|(field, ty)| {
            quote! {
                let #field = <#ty as #trait_path>::decode_raw(ctx);
            }
        });

    let expanded = quote! {
        impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #size_terms)*;

            fn decode_raw(ctx: &#ctx_path) -> Self {
                #(#decode_stmts)*
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    Ok(expanded)
}
