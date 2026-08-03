use quote::quote;
use syn::{Data, DeriveInput, Fields, GenericParam, Lifetime, LifetimeParam, spanned::Spanned};

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

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

    // Figure out which lifetime ties Self to the `&DecodeCtx` input:
    // - struct already has one (e.g. `Wrapper<'a>`) -> reuse it
    // - struct has none -> introduce a fresh impl-only lifetime
    // - struct has more than one -> unsupported, bail with a clear error
    let existing_lifetimes: Vec<_> = input.generics.lifetimes().cloned().collect();

    let mut impl_generics_src = input.generics.clone();
    let ctx_lifetime: Lifetime = match existing_lifetimes.len() {
        0 => {
            let lt = Lifetime::new("'__decode", proc_macro2::Span::call_site());
            impl_generics_src
                .params
                .insert(0, GenericParam::Lifetime(LifetimeParam::new(lt.clone())));
            lt
        }
        1 => existing_lifetimes[0].lifetime.clone(),
        _ => {
            return Err(syn::Error::new(
                input.generics.span(),
                "DecodableV2 can only be derived for structs with zero or one lifetime parameter",
            ));
        }
    };

    // impl-side generics may include the freshly injected lifetime;
    // Self's type generics must not, so these are derived separately.
    let (impl_generics, _, where_clause) = impl_generics_src.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();

    // Fully-qualified path to the trait, so callers never need to import it.
    let trait_path = quote! { crate::input_processor::DecodableV2 };
    let ctx_path = quote! { crate::input_processor::DecodeCtx };

    let size_terms = field_types
        .iter()
        .map(|ty| quote! { <#ty as #trait_path<#ctx_lifetime>>::ENCODED_SIZE });

    let decode_stmts = field_names
        .iter()
        .zip(field_types.iter())
        .map(|(field, ty)| {
            quote! {
                let #field = <#ty as #trait_path<#ctx_lifetime>>::decode_raw(ctx);
            }
        });

    let expanded = quote! {
        impl #impl_generics #trait_path<#ctx_lifetime> for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #size_terms)*;

            fn decode_raw(ctx: &#ctx_lifetime #ctx_path) -> Self {
                #(#decode_stmts)*
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    Ok(expanded)
}
