pub mod args_buffer_v2;
mod args_writer;
pub mod traits;

pub use args_buffer_v2::*;
pub use args_writer::*;
use goblin_hostio::hostio_unsafe;
pub use traits::*;

use core::cell::Cell;
use core::mem::MaybeUninit;

pub const INPUT_SIZE: usize = 512;

pub struct ArgsReader {
    pub args: [u8; INPUT_SIZE],
    pub len: usize,
    pub offset: Cell<usize>,
}

impl ArgsReader {
    pub fn new(len: usize) -> Self {
        let mut args_buffer = MaybeUninit::<[u8; INPUT_SIZE]>::uninit();
        let args = unsafe {
            hostio_unsafe::read_args(args_buffer.as_mut_ptr() as *mut u8);
            args_buffer.assume_init()
        };

        Self {
            args,
            len,
            offset: Cell::<usize>::default(),
        }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut args = [0u8; crate::input_processor::INPUT_SIZE];
        let len = slice.len().min(crate::input_processor::INPUT_SIZE);
        args[..len].copy_from_slice(&slice[..len]);
        Self {
            args,
            len: slice.len(),
            offset: Cell::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn advance_offset(&self, increment: usize) {
        let current_offset = self.offset.get();
        self.offset.set(current_offset + increment);
    }
}
