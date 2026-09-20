mod impl_checked_ops;
mod impl_const_default;
mod impl_div;
mod impl_from;
mod impl_mul;
mod impl_store_reader;
mod impl_try_from;
mod impl_unside_quantity;

use core::marker::PhantomData;

use deku::no_std_io::{Read, Seek};
use deku::reader::Reader;
use deku::{DekuError, DekuReader};

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tuple<T0, T1, K>(pub T0, pub T1, PhantomData<K>);

impl<T0, T1, K> Tuple<T0, T1, K> {
    pub const fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

// `Tuple` is used as a byte-aligned wire field (e.g. `SamePair`, `TokenIndexPair`),
// so it must be readable by deku. The marker `K` is not read, which is why this is
// a hand-written impl: a `#[derive(DekuRead)]` would demand `K: DekuRead`.
impl<'a, T0, T1, K> DekuReader<'a, ()> for Tuple<T0, T1, K>
where
    T0: DekuReader<'a, ()>,
    T1: DekuReader<'a, ()>,
{
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _ctx: (),
    ) -> Result<Self, DekuError> {
        let t0 = T0::from_reader_with_ctx(reader, ())?;
        let t1 = T1::from_reader_with_ctx(reader, ())?;
        Ok(Self::new(t0, t1))
    }
}

/// A scalar that can be decoded from, and reports the width of, a raw bit field.
///
/// It carries just the information a `Tuple` needs to split a raw field into its
/// two components (`T0` in the low `WIDTH` bits, then `T1`).
pub trait RawField: Sized {
    /// Number of bits this value occupies when packed into a raw bit field.
    const WIDTH: u64;

    /// Decode from the low bits of `raw`.
    fn from_raw(raw: u64) -> Self;
}

impl RawField for bool {
    const WIDTH: u64 = 1;

    #[inline]
    fn from_raw(raw: u64) -> Self {
        raw & 1 != 0
    }
}

impl<T0, T1, K> Tuple<T0, T1, K>
where
    T0: RawField,
    T1: RawField,
{
    /// Decode a tuple packed as `T0` followed by `T1` in the next `T0::WIDTH` bits.
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self::new(T0::from_raw(raw), T1::from_raw(raw >> T0::WIDTH))
    }
}
