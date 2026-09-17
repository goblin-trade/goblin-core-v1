use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, Index, LitInt, Type, spanned::Spanned,
};

fn fixed() -> TokenStream {
    quote! { crate::input_processor::FixedCodec }
}

fn pack() -> TokenStream {
    quote! { crate::input_processor::BitPack }
}

/// A field plus the bindings/attributes used to generate its codec.
struct Field {
    /// Local binding inside `raw_fixed_decode` (`a`, `field_0`, ...).
    binding: syn::Ident,
    /// Accessor on the value (`self.a`, `self.0`, ...).
    accessor: TokenStream,
    ty: Type,
    /// Sub-byte width from `#[codec(bits = N)]`.
    bits: Option<u8>,
    /// Byte-aligned wire type from `#[codec(wire = T)]`, cast through it.
    wire: Option<Type>,
}

/// One unit of generated work, in wire order.
enum Op {
    /// A byte-aligned field handled directly by `FixedCodec`.
    Full(usize),
    /// A run of sub-byte fields packed into one lane: `(field, shift, bits)`.
    Lane {
        width: usize,
        members: Vec<(usize, u8, u8)>,
    },
}

/// Generated statements and size terms for an `impl` body.
#[derive(Default)]
struct Codegen {
    decode: Vec<TokenStream>,
    encode: Vec<TokenStream>,
    validate: Vec<TokenStream>,
    /// Terms summed for `ENCODED_SIZE`.
    sizes: Vec<TokenStream>,
}

fn err(span: Span, msg: impl core::fmt::Display) -> syn::Error {
    syn::Error::new(span, msg)
}

fn is_bool(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.qself.is_none() && p.path.is_ident("bool"))
}

/// Smallest lane width in bytes that holds `bits` bits.
fn lane_bytes(bits: u16) -> usize {
    match bits {
        0..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        _ => 8,
    }
}

fn lane_ty(width: usize) -> TokenStream {
    match width {
        1 => quote! { u8 },
        2 => quote! { u16 },
        4 => quote! { u32 },
        _ => quote! { u64 },
    }
}

/// Parse `#[codec(bits = N)]` / `#[codec(wire = T)]` from a field.
fn parse_field_attrs(attrs: &[Attribute]) -> syn::Result<(Option<u8>, Option<Type>)> {
    let (mut bits, mut wire): (Option<u8>, Option<Type>) = (None, None);
    for attr in attrs.iter().filter(|a| a.path().is_ident("codec")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("bits") {
                bits = Some(meta.value()?.parse::<LitInt>()?.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("wire") {
                wire = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("expected `bits = N` or `wire = T`"))
            }
        })?;
    }
    Ok((bits, wire))
}

/// Build [`Field`]s from a struct's fields. The bool is `is_tuple`.
fn collect_fields(data: &DataStruct) -> syn::Result<(Vec<Field>, bool)> {
    let mut fields = Vec::new();
    match &data.fields {
        Fields::Named(named) => {
            for f in &named.named {
                let id = f.ident.clone().expect("named field");
                let (bits, wire) = parse_field_attrs(&f.attrs)?;
                fields.push(Field {
                    accessor: quote! { self.#id },
                    binding: id,
                    ty: f.ty.clone(),
                    bits,
                    wire,
                });
            }
            Ok((fields, false))
        }
        Fields::Unnamed(unnamed) => {
            for (i, f) in unnamed.unnamed.iter().enumerate() {
                let idx = Index::from(i);
                let (bits, wire) = parse_field_attrs(&f.attrs)?;
                fields.push(Field {
                    // Synthetic binding, only valid inside `raw_fixed_decode`.
                    binding: format_ident!("field_{i}"),
                    accessor: quote! { self.#idx },
                    ty: f.ty.clone(),
                    bits,
                    wire,
                });
            }
            Ok((fields, true))
        }
        Fields::Unit => Err(err(
            data.fields.span(),
            "FixedCodec requires at least one field",
        )),
    }
}

/// `bool` is always one bit; reject a different width or a `wire` override.
fn normalize_bools(fields: &mut [Field]) -> syn::Result<()> {
    for f in fields.iter_mut().filter(|f| is_bool(&f.ty)) {
        if f.wire.is_some() {
            return Err(err(f.ty.span(), "`bool` fields cannot take `wire = T`"));
        }
        if let Some(bits) = f.bits
            && bits != 1
        {
            return Err(err(
                f.ty.span(),
                "`bool` fields are always 1 bit; remove `bits = N`",
            ));
        }
        f.bits = Some(1);
    }
    Ok(())
}

/// Group fields into byte-aligned `Full` fields and sub-byte `Lane` runs.
///
/// Sub-byte fields pack LSB-first into a lane (matching the hand-written
/// decoders); a lane flushes before a full-width field or when adding the next
/// field would exceed 64 bits.
fn plan(fields: &[Field]) -> syn::Result<Vec<Op>> {
    fn flush(ops: &mut Vec<Op>, cur: &mut Vec<(usize, u8, u8)>, used: &mut u16) {
        if !cur.is_empty() {
            ops.push(Op::Lane {
                width: lane_bytes(*used),
                members: core::mem::take(cur),
            });
            *used = 0;
        }
    }

    let (mut ops, mut cur, mut used) = (Vec::new(), Vec::new(), 0u16);
    for (i, f) in fields.iter().enumerate() {
        if f.wire.is_some() && f.bits.is_some() {
            return Err(err(
                f.ty.span(),
                "`wire = T` and `bits = N` are mutually exclusive",
            ));
        }
        match f.bits.filter(|b| *b > 0 && f.wire.is_none()) {
            Some(bits) => {
                if bits > 64 {
                    return Err(err(f.ty.span(), "`bits = N` supports at most 64 bits"));
                }
                if used + bits as u16 > 64 {
                    flush(&mut ops, &mut cur, &mut used);
                }
                cur.push((i, used as u8, bits));
                used += bits as u16;
            }
            None => {
                flush(&mut ops, &mut cur, &mut used);
                ops.push(Op::Full(i));
            }
        }
    }
    flush(&mut ops, &mut cur, &mut used);
    Ok(ops)
}

/// Codegen without a top-level size: annotated fields pack into lanes,
/// unannotated fields stay byte-aligned.
fn codegen_plan(fields: &[Field]) -> syn::Result<Codegen> {
    let (fixed, pack) = (fixed(), pack());
    let mut cod = Codegen::default();

    for (n, op) in plan(fields)?.iter().enumerate() {
        match op {
            Op::Full(i) => {
                let f = &fields[*i];
                let (ty, binding, accessor) = (&f.ty, &f.binding, &f.accessor);
                // `wire = T` casts through `T`; otherwise the field type is used.
                let (codec, decode, value) = match &f.wire {
                    Some(w) => (
                        quote! { #w },
                        quote! { let #binding = <#w as #fixed>::raw_fixed_decode(reader) as #ty; },
                        quote! { (#accessor as #w) },
                    ),
                    None => (
                        quote! { #ty },
                        quote! { let #binding = <#ty as #fixed>::raw_fixed_decode(reader); },
                        quote! { #accessor },
                    ),
                };
                cod.decode.push(decode);
                cod.encode
                    .push(quote! { <#codec as #fixed>::raw_fixed_encode(&#value, writer); });
                cod.validate
                    .push(quote! { <#codec as #fixed>::validate(&#value)?; });
                cod.sizes.push(quote! { <#codec as #fixed>::ENCODED_SIZE });
            }
            Op::Lane { width, members } => {
                let (lane, var) = (lane_ty(*width), format_ident!("lane_{n}"));
                cod.decode.push(
                    quote! { let #var = <#lane as #fixed>::raw_fixed_decode(reader) as u64; },
                );
                cod.encode.push(quote! { let mut #var: u64 = 0; });
                for (i, shift, bits) in members {
                    let f = &fields[*i];
                    let (ty, binding, accessor) = (&f.ty, &f.binding, &f.accessor);
                    cod.decode.push(
                        quote! { let #binding = <#ty as #pack>::unpack_from(#var, #shift, #bits); },
                    );
                    cod.encode.push(
                        quote! { <#ty as #pack>::pack_into(&#accessor, &mut #var, #shift, #bits); },
                    );
                    cod.validate
                        .push(quote! { <#ty as #pack>::validate(&#accessor)?; });
                }
                cod.encode.push(
                    quote! { <#lane as #fixed>::raw_fixed_encode(&(#var as #lane), writer); },
                );
                cod.sizes.push(quote! { #width });
            }
        }
    }
    Ok(cod)
}

/// Codegen for a top-level `#[codec(bits = N)]`: the struct is one little-endian
/// bit stream of `N` bits. Widths come from an explicit `bits = M`, else the
/// type's `BitPack::CAPACITY`; the last unbounded field absorbs the remainder.
fn codegen_bit_region(fields: &[Field], total: u8) -> syn::Result<Codegen> {
    let pack = pack();
    let byte_len = (total as usize).div_ceil(8);

    if let Some(f) = fields.iter().find(|f| f.wire.is_some()) {
        return Err(err(
            f.ty.span(),
            "`wire = T` cannot be used with a top-level `bits = N`",
        ));
    }
    if byte_len > 8 {
        return Err(err(
            Span::call_site(),
            "top-level `bits = N` supports at most 64 bits",
        ));
    }
    if fields.iter().all(|f| f.bits.is_some()) {
        let sum: u32 = fields.iter().filter_map(|f| f.bits).map(u32::from).sum();
        if sum != total as u32 {
            return Err(err(
                Span::call_site(),
                format!("field widths sum to {sum} bits but `bits = {total}` was declared"),
            ));
        }
    }

    let last = fields.len() - 1;
    let mut cod = Codegen {
        decode: vec![
            quote! { let __lane = crate::input_processor::read_lane(reader, #byte_len); let mut __shift: u8 = 0; },
        ],
        encode: vec![quote! { let mut __lane: u64 = 0; let mut __shift: u8 = 0; }],
        ..Default::default()
    };

    for (i, f) in fields.iter().enumerate() {
        let (ty, binding, accessor) = (&f.ty, &f.binding, &f.accessor);
        let w = format_ident!("__width_{i}");
        let width = match f.bits {
            Some(bits) => quote! { #bits },
            None if i == last => quote! { (#total as u8) - __shift },
            None => quote! { <#ty as #pack>::CAPACITY },
        };
        cod.decode.push(quote! { let #w: u8 = #width; let #binding = <#ty as #pack>::unpack_from(__lane, __shift, #w); __shift += #w; });
        cod.encode.push(quote! { let #w: u8 = #width; <#ty as #pack>::pack_into(&#accessor, &mut __lane, __shift, #w); __shift += #w; });
        cod.validate
            .push(quote! { <#ty as #pack>::validate(&#accessor)?; });
    }

    cod.decode
        .push(quote! { debug_assert_eq!(__shift, #total); });
    cod.encode.push(quote! { debug_assert_eq!(__shift, #total); crate::input_processor::write_lane(writer, __lane, #byte_len); });
    cod.sizes.push(quote! { #byte_len });
    Ok(cod)
}

/// `#[fixed_codec(bits = N, validate = path)]`: re-emit the struct (without the
/// `codec` helper attributes) and append the generated `impl`.
pub fn expand_attribute(
    mut item: syn::ItemStruct,
    bits: Option<u8>,
    validate: Option<syn::Path>,
) -> syn::Result<TokenStream> {
    let input = DeriveInput {
        attrs: item.attrs.clone(),
        vis: item.vis.clone(),
        ident: item.ident.clone(),
        generics: item.generics.clone(),
        data: Data::Struct(DataStruct {
            struct_token: item.struct_token,
            fields: item.fields.clone(),
            semi_token: item.semi_token,
        }),
    };

    let impl_tokens = expand_with_bits(input, bits, validate)?;

    item.attrs.retain(|a| !a.path().is_ident("codec"));
    strip_codec_attrs(&mut item.fields);
    Ok(quote! { #item #impl_tokens })
}

/// Remove `#[codec(...)]` from every field in place.
fn strip_codec_attrs(fields: &mut Fields) {
    let attrs: Vec<&mut Vec<Attribute>> = match fields {
        Fields::Named(named) => named.named.iter_mut().map(|f| &mut f.attrs).collect(),
        Fields::Unnamed(unnamed) => unnamed.unnamed.iter_mut().map(|f| &mut f.attrs).collect(),
        Fields::Unit => Vec::new(),
    };
    for a in attrs {
        a.retain(|attr| !attr.path().is_ident("codec"));
    }
}

fn expand_with_bits(
    input: DeriveInput,
    struct_bits: Option<u8>,
    validate_override: Option<syn::Path>,
) -> syn::Result<TokenStream> {
    let data = match &input.data {
        Data::Struct(data) => data,
        Data::Enum(data) => {
            return Err(err(
                data.enum_token.span(),
                "FixedCodec cannot be derived for enums",
            ));
        }
        Data::Union(data) => {
            return Err(err(
                data.union_token.span(),
                "FixedCodec cannot be derived for unions",
            ));
        }
    };

    let (mut fields, is_tuple) = collect_fields(data)?;
    normalize_bools(&mut fields)?;
    if fields.is_empty() {
        return Err(err(
            input.ident.span(),
            "FixedCodec requires at least one field",
        ));
    }

    // A top-level size switches to the single bit-region model.
    let codegen = match struct_bits {
        Some(total) => codegen_bit_region(&fields, total)?,
        None => codegen_plan(&fields)?,
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;
    let sizes = &codegen.sizes;
    let (decode, encode) = (&codegen.decode, &codegen.encode);
    let validate = match &validate_override {
        Some(path) => quote! { #path(self) },
        None => {
            let checks = &codegen.validate;
            quote! { #(#checks)* Ok(()) }
        }
    };
    let bindings = fields.iter().map(|f| &f.binding);
    let constructor = if is_tuple {
        quote! { Self(#(#bindings),*) }
    } else {
        quote! { Self { #(#bindings),* } }
    };

    Ok(quote! {
        impl #impl_generics crate::input_processor::FixedCodec for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #sizes)*;

            fn raw_fixed_decode(reader: &crate::input_processor::ArgsReader) -> Self {
                #(#decode)*
                #constructor
            }

            fn raw_fixed_encode(&self, writer: &mut crate::input_processor::ArgsWriter) {
                #(#encode)*
            }

            fn validate(&self) -> Result<(), crate::goblin_error::GoblinError> {
                #validate
            }
        }
    })
}

/// Arguments for `#[fixed_codec(bits = N, validate = path)]`.
pub struct BitsArgs {
    pub bits: Option<u8>,
    pub validate: Option<syn::Path>,
}

impl syn::parse::Parse for BitsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let (mut bits, mut validate) = (None, None);
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            if ident == "bits" {
                bits = Some(input.parse::<LitInt>()?.base10_parse()?);
            } else if ident == "validate" {
                validate = Some(input.parse()?);
            } else {
                return Err(err(
                    ident.span(),
                    "expected `bits = N` or `validate = path`",
                ));
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(BitsArgs { bits, validate })
    }
}
