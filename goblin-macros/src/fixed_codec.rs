use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, Index, LitInt, Type, spanned::Spanned};

/// Per-field information gathered from the struct definition.
struct FieldInfo {
    /// Binding used inside the generated `raw_fixed_decode` (`a`, `field_0`, ...).
    binding: syn::Ident,
    /// Accessor on the constructed value (`self.a`, `self.0`, ...), used by
    /// encode and validate.
    accessor: proc_macro2::TokenStream,
    ty: Type,
    /// Explicit wire width from `#[codec(bits = N)]`, if present. When `None`
    /// the field is byte-aligned and goes through `FixedCodec` directly.
    bits: Option<u8>,
}

/// A member of a sub-byte lane, at a fixed bit offset within the lane.
struct LaneMember {
    field: usize,
    shift: u8,
    bits: u8,
}

/// A unit of generated work, in wire order.
enum Op {
    /// A byte-aligned field, handled directly by `FixedCodec`.
    Full(usize),
    /// A run of sub-byte fields packed into one integer lane.
    Lane {
        width_bytes: usize,
        members: Vec<LaneMember>,
    },
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

/// Smallest lane type that can hold `bits` bits.
fn lane_width_bytes(bits: u16) -> usize {
    match bits {
        0..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        _ => 8,
    }
}

/// Group fields into byte-aligned `Full` fields and sub-byte `Lane` runs.
///
/// Sub-byte fields are packed LSB-first into a lane, matching the hand-written
/// decoders (e.g. `byte_0 & 0b0000_0001` is the first field). A lane is flushed
/// when a full-width field follows, or when adding the next field would exceed
/// 64 bits.
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

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let (fields, is_tuple): (Vec<FieldInfo>, bool) = match &input.data {
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

    let ops = plan(&fields)?;

    // No lifetime threading is needed: `FixedCodec` deals in owned, fixed-size
    // values, so the struct's own generics (including `PhantomData` type
    // parameters) are used as-is for both the impl and `Self`.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Fully-qualified paths so callers never need to import the traits.
    let trait_path = quote! { crate::input_processor::FixedCodec };
    let bit_pack_path = quote! { crate::input_processor::BitPack };
    let reader_path = quote! { crate::input_processor::ArgsReader };
    let writer_path = quote! { crate::input_processor::ArgsWriter };
    let error_path = quote! { crate::goblin_error::GoblinError };

    let mut decode_stmts = Vec::new();
    let mut encode_stmts = Vec::new();
    let mut validate_stmts = Vec::new();
    let mut size_terms = Vec::new();

    for (op_index, op) in ops.iter().enumerate() {
        match op {
            Op::Full(index) => {
                let field = &fields[*index];
                let ty = &field.ty;
                let binding = &field.binding;
                let accessor = &field.accessor;

                decode_stmts.push(quote! {
                    let #binding = <#ty as #trait_path>::raw_fixed_decode(reader);
                });
                encode_stmts.push(quote! {
                    <#ty as #trait_path>::raw_fixed_encode(&#accessor, writer);
                });
                validate_stmts.push(quote! {
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

                // Decode the whole lane, then slice its bits into fields.
                decode_stmts.push(quote! {
                    let #lane_var =
                        <#lane_ty as #trait_path>::raw_fixed_decode(reader) as u64;
                });
                for member in members {
                    let field = &fields[member.field];
                    let ty = &field.ty;
                    let binding = &field.binding;
                    let shift = member.shift;
                    let bits = member.bits;
                    decode_stmts.push(quote! {
                        let #binding =
                            <#ty as #bit_pack_path>::unpack_from(#lane_var, #shift, #bits);
                    });
                }

                // Build the lane from fields, then encode it as one integer.
                encode_stmts.push(quote! {
                    let mut #lane_var: u64 = 0;
                });
                for member in members {
                    let field = &fields[member.field];
                    let ty = &field.ty;
                    let accessor = &field.accessor;
                    let shift = member.shift;
                    let bits = member.bits;
                    encode_stmts.push(quote! {
                        <#ty as #bit_pack_path>::pack_into(
                            &#accessor, &mut #lane_var, #shift, #bits,
                        );
                    });
                    validate_stmts.push(quote! {
                        <#ty as #bit_pack_path>::validate(&#accessor)?;
                    });
                }
                encode_stmts.push(quote! {
                    <#lane_ty as #trait_path>::raw_fixed_encode(
                        &(#lane_var as #lane_ty), writer,
                    );
                });

                size_terms.push(quote! { #width_bytes });
            }
        }
    }

    // Named structs build `Self { a, b, c }`; tuple structs build
    // `Self(field_0, field_1, ...)`. Field order matches declaration order,
    // which is what makes the sequential lane planning correct.
    let constructor = if is_tuple {
        let bindings = fields.iter().map(|f| &f.binding);
        quote! { Self(#(#bindings),*) }
    } else {
        let names = fields
            .iter()
            .map(|f| {
                let ident = &f.binding;
                quote! { #ident }
            })
            .collect::<Vec<_>>();
        quote! { Self { #(#names),* } }
    };

    let expanded = quote! {
        impl #impl_generics #trait_path for #name #ty_generics #where_clause {
            const ENCODED_SIZE: usize = 0 #(+ #size_terms)*;

            fn raw_fixed_decode(reader: &#reader_path) -> Self {
                #(#decode_stmts)*
                #constructor
            }

            fn raw_fixed_encode(&self, writer: &mut #writer_path) {
                #(#encode_stmts)*
            }

            fn validate(&self) -> Result<(), #error_path> {
                #(#validate_stmts)*
                Ok(())
            }
        }
    };

    Ok(expanded)
}
