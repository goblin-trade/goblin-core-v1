use crate::input_processor::ArgsBuffer;

pub struct DecodeCtx<'a> {
    pub args: &'a ArgsBuffer,
    pub offset: &'a mut usize,
    pub len: usize,
}
