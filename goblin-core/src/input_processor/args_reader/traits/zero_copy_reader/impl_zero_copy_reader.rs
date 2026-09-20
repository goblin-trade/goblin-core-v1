use crate::input_processor::{ArgsReader, ZeroCopyReader};

impl ZeroCopyReader for ArgsReader {
    fn zero_copy_unchecked<T>(&self) -> &T {
        let start = self.offset.get();
        let end = start + core::mem::size_of::<T>();
        self.offset.set(end);

        unsafe { &*(self.args[start..end].as_ptr() as *const T) }
    }

    fn zero_copy_slice_unchecked<T>(&self, slice_len: usize) -> &[T] {
        let start = self.offset.get();
        let end = start + slice_len * core::mem::size_of::<T>();
        self.offset.set(end);

        unsafe {
            core::slice::from_raw_parts(self.args[start..end].as_ptr() as *const T, slice_len)
        }
    }
}
