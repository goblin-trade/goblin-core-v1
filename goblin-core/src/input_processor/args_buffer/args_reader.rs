use crate::codec::Reader;
use crate::input_processor::ArgsBuffer;

pub type ArgsReader<'de> = Reader<'de>;

impl<'de> From<&'de ArgsBuffer> for ArgsReader<'de> {
    fn from(value: &'de ArgsBuffer) -> Self {
        Reader::new(&value.inner)
    }
}
