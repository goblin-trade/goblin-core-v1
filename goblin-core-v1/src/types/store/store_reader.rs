/// A marker type to read fields from a store at compile time
pub trait StoreReader<S> {
    type Result: Clone + Copy;

    fn get(store: &S) -> Self::Result;
    fn get_leg(store: &S) -> &Self::Result;
    fn get_leg_mut(store: &mut S) -> &mut Self::Result;
}
