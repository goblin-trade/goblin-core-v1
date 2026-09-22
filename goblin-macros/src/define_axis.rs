use std::collections::HashSet;

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Attribute, Ident, ItemEnum, Lit, Token, parse::Parser, spanned::Spanned};

/// Baseline derive set applied to every generated axis enum. Extra traits can
/// be added per-enum by deriving them on the annotated item (e.g. `DekuRead`);
/// any trait the caller already derives is skipped here to avoid duplicate
/// impls.
const BASE_DERIVES: &[&str] = &[
    "Debug",
    "Clone",
    "Copy",
    "PartialEq",
    "PartialOrd",
    "Eq",
    "Ord",
];

/// Expands `#[define_axis]` on an axis enum.
///
/// The enum keeps all of its own attributes (doc comments, `cfg_attr`, extra
/// derives such as Deku, ...) and gains:
///
/// * a seed struct named after the enum with the trailing `Enum` removed
///   (e.g. `OccupancyEnum` -> `Occupancy`), used only to parameterise
///   `crate::types::Marker`,
/// * `From<bool>` (binary axes only), `TryFrom<u8>` and `from_raw`,
/// * one `Marker` type alias per variant, plus its `AxisMarker` impl.
pub fn expand(attr: TokenStream, mut item: ItemEnum) -> syn::Result<TokenStream> {
    if item.variants.is_empty() {
        return Err(syn::Error::new(
            item.ident.span(),
            "`define_axis` requires at least one variant",
        ));
    }

    let enum_ident = item.ident.clone();
    let enum_vis = item.vis.clone();
    let seed_ident = match parse_seed_override(attr)? {
        Some(ident) => ident,
        None => seed_from_enum(&enum_ident)?,
    };

    // Resolve every variant to a concrete integer discriminant, honouring both
    // the explicit `= N` and the implicit (previous + 1) forms.
    let mut values: Vec<Literal> = Vec::with_capacity(item.variants.len());
    let variant_idents: Vec<Ident> = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();
    let mut next: u64 = 0;
    for variant in item.variants.iter() {
        let value = match &variant.discriminant {
            Some((_, expr)) => match expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(int), ..
                }) => int.base10_parse::<u64>()?,
                other => {
                    return Err(syn::Error::new(
                        other.span(),
                        "`define_axis` requires integer literal discriminants",
                    ));
                }
            },
            None => next,
        };
        next = value + 1;
        values.push(Literal::u64_unsuffixed(value));
    }

    // Rebuild the enum's attributes: our baseline derives/repr first, then the
    // caller's own attributes verbatim.
    let user_derived = collect_derived_traits(&item.attrs);
    let missing_derives: Vec<Ident> = BASE_DERIVES
        .iter()
        .filter(|trait_name| !user_derived.contains(**trait_name))
        .map(|trait_name| Ident::new(trait_name, Span::call_site()))
        .collect();

    let mut attrs: Vec<Attribute> = Vec::new();
    if !missing_derives.is_empty() {
        attrs.push(syn::parse_quote!(#[derive(#(#missing_derives),*)]));
    }
    if !item.attrs.iter().any(|a| a.path().is_ident("repr")) {
        attrs.push(syn::parse_quote!(#[repr(u8)]));
    }
    attrs.extend(item.attrs.iter().cloned());
    item.attrs = attrs;

    // `From<bool>` only makes sense for a binary axis.
    let from_bool = if values.len() == 2 {
        let v0 = &variant_idents[0];
        let v1 = &variant_idents[1];
        quote! {
            impl ::core::convert::From<bool> for #enum_ident {
                fn from(value: bool) -> Self {
                    if value { Self::#v1 } else { Self::#v0 }
                }
            }
        }
    } else {
        TokenStream::new()
    };

    // `from_raw` folds the low bits onto the variant range, mirroring the old
    // hand-written `raw & 1` / `raw & 0b11` forms. The wildcard arm is always
    // required: the scrutinee is a `u64`, so a power-of-two mask still leaves
    // every higher pattern value uncovered as far as the type checker is
    // concerned.
    let mask = Literal::u64_unsuffixed(values.len().next_power_of_two() as u64 - 1);
    let last = &variant_idents[values.len() - 1];

    let from_raw_arms = values
        .iter()
        .zip(&variant_idents)
        .map(|(value, ident)| quote! { #value => Self::#ident, });
    let try_from_arms = values
        .iter()
        .zip(&variant_idents)
        .map(|(value, ident)| quote! { #value => Ok(Self::#ident), });

    let markers = variant_idents.iter().map(|ident| {
        quote! {
            pub type #ident = crate::types::Marker<#seed_ident, { #enum_ident::#ident as usize }>;

            impl crate::axis_helpers::AxisMarker for #ident {
                type Axis = #seed_ident;
                type Enum = #enum_ident;
                const VARIANT: Self::Enum = #enum_ident::#ident;
            }
        }
    });

    Ok(quote! {
        #[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
        #enum_vis struct #seed_ident;

        #item

        #from_bool

        impl ::core::convert::TryFrom<u8> for #enum_ident {
            type Error = crate::goblin_error::GoblinError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    #(#try_from_arms)*
                    _ => Err(crate::goblin_error::GoblinError::InvalidEnumVariant),
                }
            }
        }

        impl #enum_ident {
            #[inline]
            pub const fn from_raw(raw: u64) -> Self {
                match raw & #mask {
                    #(#from_raw_arms)*
                    _ => Self::#last,
                }
            }
        }

        #(#markers)*
    })
}

/// Parses an optional `seed = Ident` argument from the attribute. The seed
/// struct name otherwise defaults to the enum name minus its `Enum` suffix.
fn parse_seed_override(attr: TokenStream) -> syn::Result<Option<Ident>> {
    if attr.is_empty() {
        return Ok(None);
    }

    let parser = |input: syn::parse::ParseStream| {
        let path: syn::Path = input.parse()?;
        if !path.is_ident("seed") {
            return Err(syn::Error::new(
                path.span(),
                "unsupported `define_axis` argument; expected `seed = Ident`",
            ));
        }
        input.parse::<Token![=]>()?;
        let ident: Ident = input.parse()?;
        Ok(ident)
    };

    Ok(Some(parser.parse2(attr)?))
}

fn seed_from_enum(enum_ident: &Ident) -> syn::Result<Ident> {
    let name = enum_ident.to_string();
    match name.strip_suffix("Enum") {
        Some(seed) if !seed.is_empty() => Ok(Ident::new(seed, enum_ident.span())),
        _ => Err(syn::Error::new(
            enum_ident.span(),
            "`define_axis` derives the seed name by stripping an `Enum` suffix; \
             rename the enum or pass `#[define_axis(seed = Name)]`",
        )),
    }
}

/// Collects the trait idents a set of attributes derives, so we don't emit a
/// conflicting impl for a trait the caller already asked for.
fn collect_derived_traits(attrs: &[Attribute]) -> HashSet<String> {
    let mut set = HashSet::new();
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                set.insert(ident.to_string());
            }
            Ok(())
        });
    }
    set
}
