use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, GenericParam, Index, Lifetime, LifetimeParam, spanned::Spanned,
};

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let (field_names, field_types, is_tuple): (Vec<_>, Vec<_>, bool) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let names = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().expect("named field"))
                    .collect();
                let types = fields.named.iter().map(|f| f.ty.clone()).collect();
                (names, types, false)
            }
            Fields::Unnamed(fields) => {
                // Tuple structs have no field idents, so synthesize bindings
                // (`field_0`, `field_1`, ...) to use in the decode statements
                // and in the final `Self(...)` constructor. Note these
                // synthetic names are only valid inside `raw_fixed_decode`;
                // `validate` runs on `&self` after construction and must
                // access fields via `self.0`, `self.1`, ... instead (see
                // `field_accessors` below).
                let names = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field_{}", i))
                    .collect();
                let types = fields.unnamed.iter().map(|f| f.ty.clone()).collect();
                (names, types, true)
            }
            Fields::Unit => {
                return Err(syn::Error::new(
                    data.fields.span(),
                    "FixedDecode can only be derived for structs with at least one field",
                ));
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span(),
                "FixedDecode cannot be derived for enums",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "FixedDecode cannot be derived for unions",
            ));
        }
    };

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
                "FixedDecode can only be derived for structs with zero or one lifetime parameter",
            ));
        }
    };

    // impl-side generics may include the freshly injected lifetime;
    // Self's type generics must not, so these are derived separately.
    let (impl_generics, _, where_clause) = impl_generics_src.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();

    // Fully-qualified path to the trait, so callers never need to import it.
    let trait_path = quote! { crate::input_processor::FixedDecode };
    let ctx_path = quote! { crate::input_processor::DecodeCtx };
    let error_path = quote! { crate::goblin_error::GoblinError };

    let size_terms = field_types
        .iter()
        .map(|ty| quote! { <#ty as #trait_path<#ctx_lifetime>>::ENCODED_SIZE });

    let decode_stmts = field_names
        .iter()
        .zip(field_types.iter())
        .map(|(field, ty)| {
            quote! {
                let #field = <#ty as #trait_path<#ctx_lifetime>>::raw_fixed_decode(ctx);
            }
        });

    // Named structs build `Self { a, b, c }`; tuple structs build
    // `Self(field_0, field_1, field_2)` — field order matches declaration
    // order in both cases, which is what makes sequential decoding correct.
    let constructor = if is_tuple {
        quote! { Self(#(#field_names),*) }
    } else {
        quote! { Self { #(#field_names),* } }
    };

    // `validate` runs on `&self` post-construction, so it can't reuse the
    // synthetic `field_N` locals from `raw_fixed_decode` — tuple fields are
    // accessed as `self.0`, `self.1`, ...; named fields as `self.<name>`.
    let field_accessors: Vec<proc_macro2::TokenStream> = if is_tuple {
        (0..field_types.len())
            .map(|i| {
                let idx = Index::from(i);
                quote! { self.#idx }
            })
            .collect()
    } else {
        field_names
            .iter()
            .map(|field| quote! { self.#field })
            .collect()
    };

    let validate_stmts = field_types
        .iter()
        .zip(field_accessors.iter())
        .map(|(ty, accessor)| {
            quote! {
                <#ty as #trait_path<#ctx_lifetime>>::validate(&#accessor)?;
            }
        });

    let expanded = quote! {
        impl #impl_generics #trait_path<#ctx_lifetime> for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #size_terms)*;

            fn raw_fixed_decode(ctx: &#ctx_lifetime #ctx_path) -> Self {
                #(#decode_stmts)*
                #constructor
            }

            fn validate(&self) -> Result<(), #error_path> {
                #(#validate_stmts)*
                Ok(())
            }
        }
    };

    Ok(expanded)
}
