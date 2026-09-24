mod alias;
mod impls;

pub mod bits_layout;
pub mod inner_val;
pub mod position_range;
pub mod safe_position;

pub use alias::*;
pub use bits_layout::*;
pub use impls::*;
pub use inner_val::*;
pub use position_range::*;
pub use safe_position::*;

use core::range::RangeInclusive;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Position<K, const BITS: u16>
where
    K: InnerVal,
{
    pub inner: K,
}

impl<'de, K, const BITS: u16> GoblinRead<'de, ()> for Position<K, BITS>
where
    K: InnerVal,
{
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let inner = K::from_reader_with_ctx(reader, ())?;
        Ok(Self { inner })
    }
}

#[cfg(feature = "encode")]
impl<K, const BITS: u16> GoblinWrite<()> for Position<K, BITS>
where
    K: InnerVal,
{
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.inner.to_writer(writer, ())
    }
}

impl<K, const BITS: u16> Position<K, BITS>
where
    K: InnerVal,
{
    pub const fn new(inner: K) -> Self {
        Self { inner }
    }

    /// Decode from the low bits of a raw bit field, delegating to the inner value.
    #[inline]
    pub fn from_raw(raw: u64) -> Self {
        Self::new(K::from_raw(raw))
    }

    pub fn convert_range(value: RangeInclusive<FullPos>) -> RangeInclusive<Self> {
        RangeInclusive {
            start: value.start.extract_and_convert(),
            last: value.last.extract_and_convert(),
        }
    }

    pub fn into_position(&self) -> FullPos {
        let inner: u64 = self.inner.into();
        FullPos {
            inner: (inner & BitsLayout::<BITS>::MASK) << BitsLayout::<BITS>::OFFSET,
        }
    }
}
