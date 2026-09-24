use crate::codec::Writer;
use crate::input_processor::ArgsBuffer;

pub type ArgsWriter<'a> = Writer<'a>;

impl<'a> From<&'a mut ArgsBuffer> for ArgsWriter<'a> {
    fn from(value: &'a mut ArgsBuffer) -> Self {
        Writer::new(&mut value.inner)
    }
}
