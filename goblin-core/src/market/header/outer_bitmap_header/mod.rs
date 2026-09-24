mod process;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

use crate::quantities::OuterBitmapIndexU32;

pub struct OuterBitmapHeader {
    pub outer_bitmap_index_u32: OuterBitmapIndexU32,
    pub inner_bitmap_count: u8,
}

impl<'de> GoblinRead<'de, ()> for OuterBitmapHeader {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let outer_bitmap_index_u32 = OuterBitmapIndexU32::from_reader_with_ctx(reader, ())?;
        let inner_bitmap_count = reader.read_u8()?;

        Ok(Self {
            outer_bitmap_index_u32,
            inner_bitmap_count,
        })
    }
}

#[cfg(feature = "encode")]
impl GoblinWrite<()> for OuterBitmapHeader {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.outer_bitmap_index_u32.to_writer(writer, ())?;
        writer.write_u8(self.inner_bitmap_count)
    }
}
