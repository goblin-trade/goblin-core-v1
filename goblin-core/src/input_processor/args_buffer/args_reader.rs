use deku::{no_std_io::Cursor, reader::Reader};

use crate::input_processor::ArgsBuffer;

pub type ArgsReader<'a> = Reader<Cursor<&'a [u8]>>;

impl<'a> From<&'a ArgsBuffer> for ArgsReader<'a> {
    fn from(value: &'a ArgsBuffer) -> Self {
        let cursor = Cursor::new(value.inner.as_ref());
        Self::new(cursor)
    }
}
