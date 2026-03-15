pub trait Bitmap<P> {
    fn active(&self, pos: P) -> bool;
}
