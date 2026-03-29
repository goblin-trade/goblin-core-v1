pub trait Bitmap<P>: PartialEq + Default {
    fn pos_active(&self, pos: P) -> bool;

    fn deactivate(&mut self, pos: P);

    fn bitmap_inactive(&self) -> bool {
        *self == Self::default()
    }
}
