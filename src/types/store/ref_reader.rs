pub trait RefReader<'a, S> {
    type Result: Clone + Copy;

    fn get(store: &'a S) -> Self::Result;
}
