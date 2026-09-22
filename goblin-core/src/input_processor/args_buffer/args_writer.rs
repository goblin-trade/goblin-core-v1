use deku::{no_std_io::Cursor, reader::Reader, writer::Writer};

use crate::input_processor::ArgsBuffer;

pub type ArgsWriter<'a> = Writer<Cursor<&'a mut [u8]>>;

impl<'a> From<&'a mut ArgsBuffer> for ArgsWriter<'a> {
    fn from(value: &'a mut ArgsBuffer) -> Self {
        let cursor = Cursor::new(value.inner.as_mut());
        Self::new(cursor)
    }
}
