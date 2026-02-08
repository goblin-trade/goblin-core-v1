/// A generic store whose fields can be accessed using marker types
/// at compile time
pub trait Store<S> {
    type Result: Clone + Copy;

    fn get(store: &S) -> Self::Result;
    fn get_ref(store: &S) -> &Self::Result;
    fn get_mut(store: &mut S) -> &mut Self::Result;
}
