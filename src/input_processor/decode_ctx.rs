use crate::input_processor::ArgsBuffer;
use core::cell::Cell;

pub struct DecodeCtx<'a> {
    pub args: &'a [u8],
    pub offset: Cell<usize>,
}

impl<'a> DecodeCtx<'a> {
    pub fn new(args_buffer: &'a ArgsBuffer, len: usize) -> Self {
        Self {
            args: &args_buffer[..len],
            offset: Cell::<usize>::default(),
        }
    }
}
