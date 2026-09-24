mod process;

use crate::codec::{CodecResult, GoblinRead, Reader};
#[cfg(feature = "encode")]
use crate::codec::{GoblinWrite, Writer};

use crate::quantities::OuterPos;

pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: u8,
}

impl<'de> GoblinRead<'de, ()> for InnerBitmapHeader {
    fn from_reader_with_ctx(reader: &mut Reader<'de>, (): ()) -> CodecResult<Self> {
        let outer_pos = OuterPos::from_reader_with_ctx(reader, ())?;
        let update_count = reader.read_u8()?;

        Ok(Self {
            outer_pos,
            update_count,
        })
    }
}

#[cfg(feature = "encode")]
impl GoblinWrite<()> for InnerBitmapHeader {
    fn to_writer(&self, writer: &mut Writer<'_>, (): ()) -> CodecResult<()> {
        self.outer_pos.to_writer(writer, ())?;
        writer.write_u8(self.update_count)
    }
}
