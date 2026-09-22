pub mod args_reader;
pub mod traits;
pub mod zero_copy_reader;

pub use args_reader::*;
pub use traits::*;
pub use zero_copy_reader::*;

#[cfg(feature = "encode")]
pub mod args_writer;
#[cfg(feature = "encode")]
pub use args_writer::*;

use core::mem::MaybeUninit;
use goblin_hostio::hostio_unsafe;

pub const INPUT_SIZE: usize = 512;

pub struct ArgsBuffer {
    pub inner: [u8; INPUT_SIZE],
}

impl Default for ArgsBuffer {
    fn default() -> Self {
        let mut inner_maybeuninit = MaybeUninit::<[u8; INPUT_SIZE]>::uninit();
        let inner = unsafe {
            hostio_unsafe::read_args(inner_maybeuninit.as_mut_ptr() as *mut u8);
            inner_maybeuninit.assume_init()
        };

        Self { inner }
    }
}
