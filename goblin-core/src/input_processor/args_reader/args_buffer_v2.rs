use core::mem::MaybeUninit;

use deku::{no_std_io::Cursor, reader::Reader, writer::Writer};
use goblin_hostio::hostio_unsafe;

use crate::input_processor::INPUT_SIZE;

pub type ArgsReaderV2<'a> = Reader<Cursor<&'a [u8]>>;
pub type ArgsWriterV2<'a> = Writer<Cursor<&'a mut [u8]>>;

pub struct ArgsBufferV2 {
    pub inner: [u8; INPUT_SIZE],
}

impl Default for ArgsBufferV2 {
    fn default() -> Self {
        let mut inner_maybeuninit = MaybeUninit::<[u8; INPUT_SIZE]>::uninit();
        let inner = unsafe {
            hostio_unsafe::read_args(inner_maybeuninit.as_mut_ptr() as *mut u8);
            inner_maybeuninit.assume_init()
        };

        Self { inner }
    }
}

impl<'a> From<&'a ArgsBufferV2> for ArgsReaderV2<'a> {
    fn from(value: &'a ArgsBufferV2) -> Self {
        let cursor = Cursor::new(value.inner.as_ref());
        Self::new(cursor)
    }
}

#[cfg(feature = "encode")]
impl<'a> From<&'a mut ArgsBufferV2> for ArgsWriterV2<'a> {
    fn from(value: &'a mut ArgsBufferV2) -> Self {
        let cursor = Cursor::new(value.inner.as_mut());
        Self::new(cursor)
    }
}
