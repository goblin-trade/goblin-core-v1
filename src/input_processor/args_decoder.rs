pub trait ArgsDecoder {
    fn decode_ref<T>(&self, start: usize) -> &T;
    fn decode_slice<T>(&self, start: usize, len: usize) -> &[T];
}
