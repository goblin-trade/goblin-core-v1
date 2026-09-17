use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, Index, LitInt, Type, spanned::Spanned};

/// Per-field information gathered from the struct definition.
struct FieldInfo {
    /// Binding used inside the generated `raw_fixed_decode` (`a`, `field_0`, ...).
    binding: syn::Ident,
    /// Accessor on the constructed value (`self.a`, `self.0`, ...), used by
    /// encode and validate.
    accessor: TokenStream,
    ty: Type,
    /// Explicit wire width from `#[codec(bits = N)]`, if present.
    bits: Option<u8>,
}

/// A member of a sub-byte lane, at a fixed bit offset within the lane.
struct LaneMember {
    field: usize,
    shift: u8,
    bits: u8,
}

/// A unit of generated work, in wire order, for the no-top-level-size case.
enum Op {
    /// A byte-aligned field, handled directly by `FixedCodec`.
    Full(usize),
    /// A run of sub-byte fields packed into one integer lane.
    Lane {
        width_bytes: usize,
        members: Vec<LaneMember>,
    },
}

/// Generated statements and size expression for an `impl` body.
struct Codegen {
    decode: Vec<TokenStream>,
    encode: Vec<TokenStream>,
    validate: Vec<TokenStream>,
    /// Expression for `ENCODED_SIZE`.
    encoded_size: TokenStream,
}

/// Parse `#[codec(bits = N)]` from a field's attributes.
fn parse_bits(attrs: &[Attribute]) -> syn::Result<Option<u8>> {
    let mut bits = None;
    for attr in attrs {
        if !attr.path().is_ident("codec") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("bits") {
                let lit: LitInt = meta.value()?.parse()?;
                bits = Some(lit.base10_parse::<u8>()?);
                Ok(())
            } else {
                Err(meta.error("unknown `codec` attribute; expected `bits = N`"))
            }
        })?;
    }
    Ok(bits)
}

/// Does this type syntactically name `bool`?
fn is_bool(ty: &Type) -> bool {
    matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.is_ident("bool"))
}

/// Smallest lane type that can hold `bits` bits.
fn lane_width_bytes(bits: u16) -> usize {
    match bits {
        0..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        _ => 8,
    }
}

/// `bool` is always one bit. Errors if a `bool` is given any other explicit width.
fn normalize_bools(fields: &mut [FieldInfo]) -> syn::Result<()> {
    for field in fields.iter_mut() {
        if is_bool(&field.ty) {
            if let Some(bits) = field.bits
                && bits != 1
            {
                return Err(syn::Error::new(
                    field.ty.span(),
                    "`bool` fields are always 1 bit; remove `#[codec(bits = N)]`",
                ));
            }
            field.bits = Some(1);
        }
    }
    Ok(())
}

/// Group fields into byte-aligned `Full` fields and sub-byte `Lane` runs.
///
/// Used when there is no top-level `#[codec(bits = N)]`. Sub-byte fields are
/// packed LSB-first into a lane, matching the hand-written decoders (e.g.
/// `byte_0 & 0b0000_0001` is the first field). A lane is flushed when a
/// full-width field follows, or when adding the next field would exceed 64 bits.
fn plan(fields: &[FieldInfo]) -> syn::Result<Vec<Op>> {
    let mut ops = Vec::new();
    let mut members: Vec<LaneMember> = Vec::new();
    let mut lane_bits: u16 = 0;

    fn flush(ops: &mut Vec<Op>, members: &mut Vec<LaneMember>, lane_bits: &mut u16) {
        if members.is_empty() {
            return;
        }
        ops.push(Op::Lane {
            width_bytes: lane_width_bytes(*lane_bits),
            members: core::mem::take(members),
        });
        *lane_bits = 0;
    }

    for (i, field) in fields.iter().enumerate() {
        match field.bits {
            // No explicit width, or an explicit zero-width field: byte-aligned.
            None | Some(0) => {
                flush(&mut ops, &mut members, &mut lane_bits);
                ops.push(Op::Full(i));
            }
            Some(bits) => {
                if bits > 64 {
                    return Err(syn::Error::new(
                        field.ty.span(),
                        "`#[codec(bits = N)]` supports at most 64 bits",
                    ));
                }
                if lane_bits + bits as u16 > 64 {
                    flush(&mut ops, &mut members, &mut lane_bits);
                }
                members.push(LaneMember {
                    field: i,
                    shift: lane_bits as u8,
                    bits,
                });
                lane_bits += bits as u16;
            }
        }
    }
    flush(&mut ops, &mut members, &mut lane_bits);

    Ok(ops)
}

/// Codegen for the no-top-level-size case: explicit per-field widths are packed
/// into lanes; unannotated fields stay byte-aligned.
fn codegen_plan(fields: &[FieldInfo]) -> syn::Result<Codegen> {
    let ops = plan(fields)?;

    let trait_path = quote! { crate::input_processor::FixedCodec };
    let bit_pack_path = quote! { crate::input_processor::BitPack };

    let mut cod = Codegen {
        decode: Vec::new(),
        encode: Vec::new(),
        validate: Vec::new(),
        encoded_size: TokenStream::new(),
    };
    let mut size_terms: Vec<TokenStream> = Vec::new();

    for (op_index, op) in ops.iter().enumerate() {
        match op {
            Op::Full(index) => {
                let field = &fields[*index];
                let ty = &field.ty;
                let binding = &field.binding;
                let accessor = &field.accessor;

                cod.decode.push(quote! {
                    let #binding = <#ty as #trait_path>::raw_fixed_decode(reader);
                });
                cod.encode.push(quote! {
                    <#ty as #trait_path>::raw_fixed_encode(&#accessor, writer);
                });
                cod.validate.push(quote! {
                    <#ty as #trait_path>::validate(&#accessor)?;
                });
                size_terms.push(quote! { <#ty as #trait_path>::ENCODED_SIZE });
            }
            Op::Lane {
                width_bytes,
                members,
            } => {
                let lane_ty = match width_bytes {
                    1 => quote! { u8 },
                    2 => quote! { u16 },
                    4 => quote! { u32 },
                    _ => quote! { u64 },
                };
                let lane_var = format_ident!("lane_{}", op_index);

                cod.decode.push(quote! {
                    let #lane_var =
                        <#lane_ty as #trait_path>::raw_fixed_decode(reader) as u64;
                });
                for member in members {
                    let field = &fields[member.field];
                    let ty = &field.ty;
                    let binding = &field.binding;
                    let shift = member.shift;
                    let bits = member.bits;
                    cod.decode.push(quote! {
                        let #binding =
                            <#ty as #bit_pack_path>::unpack_from(#lane_var, #shift, #bits);
                    });
                }

                cod.encode.push(quote! {
                    let mut #lane_var: u64 = 0;
                });
                for member in members {
                    let field = &fields[member.field];
                    let ty = &field.ty;
                    let accessor = &field.accessor;
                    let shift = member.shift;
                    let bits = member.bits;
                    cod.encode.push(quote! {
                        <#ty as #bit_pack_path>::pack_into(
                            &#accessor, &mut #lane_var, #shift, #bits,
                        );
                    });
                    cod.validate.push(quote! {
                        <#ty as #bit_pack_path>::validate(&#accessor)?;
                    });
                }
                cod.encode.push(quote! {
                    <#lane_ty as #trait_path>::raw_fixed_encode(
                        &(#lane_var as #lane_ty), writer,
                    );
                });
                size_terms.push(quote! { #width_bytes });
            }
        }
    }

    cod.encoded_size = quote! { 0 #(+ #size_terms)* };
    Ok(cod)
}

/// Codegen for the top-level `#[codec(bits = N)]` case.
///
/// The whole struct is one little-endian bit stream of `N` bits (padded to a
/// byte). Field widths:
///
/// * explicit `#[codec(bits = M)]` wins;
/// * otherwise the type's `BitPack::CAPACITY` is used — 1 for `bool`, and the
///   full width for a newtype over `u8`/`u16`/... This is what makes "numbers
///   before bools occupy full space";
/// * the **last** field without an explicit width absorbs the remainder
///   `N - (sum of the other widths)` — "the number after the bools or enum".
///
/// Widths and shifts are computed at runtime from associated consts, but each
/// value monomorphizes to a constant, so they fold away.
fn codegen_bit_region(fields: &[FieldInfo], total: u8) -> syn::Result<Codegen> {
    let bit_pack_path = quote! { crate::input_processor::BitPack };
    let byte_len = (total as usize).div_ceil(8);

    if byte_len > 8 {
        return Err(syn::Error::new(
            Span::call_site(),
            "top-level `#[codec(bits = N)]` supports at most 64 bits",
        ));
    }

    // If every field pins its own width there is no field left to absorb the
    // remainder, so the widths must sum to the declared size exactly.
    if fields.iter().all(|f| f.bits.is_some()) {
        let sum: u32 = fields.iter().map(|f| f.bits.unwrap_or(0) as u32).sum();
        if sum != total as u32 {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "field widths sum to {sum} bits but `#[codec(bits = {total})]` was declared"
                ),
            ));
        }
    }

    let last = fields.len() - 1;
    let mut cod = Codegen {
        decode: vec![quote! {
            let __lane = crate::input_processor::read_lane(reader, #byte_len);
            let mut __shift: u8 = 0;
        }],
        encode: vec![quote! {
            let mut __lane: u64 = 0;
            let mut __shift: u8 = 0;
        }],
        validate: Vec::new(),
        encoded_size: quote! { #byte_len },
    };

    for (i, field) in fields.iter().enumerate() {
        let ty = &field.ty;
        let binding = &field.binding;
        let accessor = &field.accessor;
        let width_var = format_ident!("__width_{}", i);

        let width = match field.bits {
            Some(bits) => quote! { #bits },
            None if i == last => quote! { (#total as u8) - __shift },
            None => quote! { <#ty as #bit_pack_path>::CAPACITY },
        };

        cod.decode.push(quote! {
            let #width_var: u8 = #width;
            let #binding = <#ty as #bit_pack_path>::unpack_from(__lane, __shift, #width_var);
            __shift += #width_var;
        });
        cod.encode.push(quote! {
            let #width_var: u8 = #width;
            <#ty as #bit_pack_path>::pack_into(&#accessor, &mut __lane, __shift, #width_var);
            __shift += #width_var;
        });
        cod.validate.push(quote! {
            <#ty as #bit_pack_path>::validate(&#accessor)?;
        });
    }

    cod.decode.push(quote! {
        debug_assert_eq!(__shift, #total);
    });
    cod.encode.push(quote! {
        debug_assert_eq!(__shift, #total);
        crate::input_processor::write_lane(writer, __lane, #byte_len);
    });

    Ok(cod)
}

/// Attribute entry point: `#[fixed_codec(bits = N)]` supplies the packed size
/// directly. Per-field `#[codec(bits = M)]` attributes are also read and
/// consumed here.
pub fn expand_attribute(
    mut item: syn::ItemStruct,
    struct_bits: Option<u8>,
) -> syn::Result<TokenStream> {
    // `#[codec(...)]` is only meaningful to this macro. It must re-emit the
    // struct, and the compiler would reject any `#[codec(...)]` left on it, so
    // build the `DeriveInput` view first and then strip those attributes from
    // the emitted item.
    let input = DeriveInput {
        attrs: item.attrs.clone(),
        vis: item.vis.clone(),
        ident: item.ident.clone(),
        generics: item.generics.clone(),
        data: Data::Struct(syn::DataStruct {
            struct_token: item.struct_token,
            fields: item.fields.clone(),
            semi_token: item.semi_token,
        }),
    };

    let impl_tokens = expand_with_bits(input, struct_bits)?;

    item.attrs.retain(|attr| !attr.path().is_ident("codec"));
    strip_codec_attrs(&mut item.fields);

    Ok(quote! {
        #item
        #impl_tokens
    })
}

/// Remove `#[codec(...)]` from every field, in place.
fn strip_codec_attrs(fields: &mut Fields) {
    fn strip(attrs: &mut Vec<Attribute>) {
        attrs.retain(|attr| !attr.path().is_ident("codec"));
    }
    match fields {
        Fields::Named(named) => {
            for field in &mut named.named {
                strip(&mut field.attrs);
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &mut unnamed.unnamed {
                strip(&mut field.attrs);
            }
        }
        Fields::Unit => {}
    }
}

fn expand_with_bits(input: DeriveInput, struct_bits: Option<u8>) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let (mut fields, is_tuple): (Vec<FieldInfo>, bool) = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .map(|f| {
                        let ident = f.ident.clone().expect("named field");
                        Ok(FieldInfo {
                            binding: ident.clone(),
                            accessor: quote! { self.#ident },
                            ty: f.ty.clone(),
                            bits: parse_bits(&f.attrs)?,
                        })
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                (fields, false)
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        let idx = Index::from(i);
                        Ok(FieldInfo {
                            // Synthetic binding; only valid inside
                            // `raw_fixed_decode`. Encode/validate use `self.N`.
                            binding: format_ident!("field_{}", i),
                            accessor: quote! { self.#idx },
                            ty: f.ty.clone(),
                            bits: parse_bits(&f.attrs)?,
                        })
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                (fields, true)
            }
            Fields::Unit => {
                return Err(syn::Error::new(
                    data.fields.span(),
                    "FixedCodec can only be derived for structs with at least one field",
                ));
            }
        },
        Data::Enum(data) => {
            return Err(syn::Error::new(
                data.enum_token.span(),
                "FixedCodec cannot be derived for enums",
            ));
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "FixedCodec cannot be derived for unions",
            ));
        }
    };

    normalize_bools(&mut fields)?;

    if fields.is_empty() {
        return Err(syn::Error::new(
            input.ident.span(),
            "FixedCodec can only be derived for structs with at least one field",
        ));
    }

    // A top-level `#[codec(bits = N)]` switches to the single bit-region model.
    let codegen = match struct_bits {
        Some(total) => codegen_bit_region(&fields, total)?,
        None => codegen_plan(&fields)?,
    };

    // No lifetime threading is needed: `FixedCodec` deals in owned, fixed-size
    // values, so the struct's own generics (including `PhantomData` type
    // parameters) are used as-is for both the impl and `Self`.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let trait_path = quote! { crate::input_processor::FixedCodec };
    let reader_path = quote! { crate::input_processor::ArgsReader };
    let writer_path = quote! { crate::input_processor::ArgsWriter };
    let error_path = quote! { crate::goblin_error::GoblinError };
    let encoded_size = &codegen.encoded_size;
    let decode = &codegen.decode;
    let encode = &codegen.encode;
    let validate = &codegen.validate;

    let constructor = if is_tuple {
        let bindings = fields.iter().map(|f| &f.binding);
        quote! { Self(#(#bindings),*) }
    } else {
        let names = fields.iter().map(|f| &f.binding);
        quote! { Self { #(#names),* } }
    };

    let expanded = quote! {
        impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = #encoded_size;

            fn raw_fixed_decode(reader: &#reader_path) -> Self {
                #(#decode)*
                #constructor
            }

            fn raw_fixed_encode(&self, writer: &mut #writer_path) {
                #(#encode)*
            }

            fn validate(&self) -> Result<(), #error_path> {
                #(#validate)*
                Ok(())
            }
        }
    };

    Ok(expanded)
}

/// Arguments for `#[fixed_codec(bits = N)]`.
pub struct BitsArgs {
    pub bits: Option<u8>,
}

impl syn::parse::Parse for BitsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(BitsArgs { bits: None });
        }
        let ident: syn::Ident = input.parse()?;
        if ident != "bits" {
            return Err(syn::Error::new(ident.span(), "expected `bits = N`"));
        }
        input.parse::<syn::Token![=]>()?;
        let lit: LitInt = input.parse()?;
        Ok(BitsArgs {
            bits: Some(lit.base10_parse()?),
        })
    }
}
