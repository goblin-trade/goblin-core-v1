use crate::state::bitmap::Bitmap;

pub struct IntermediateIterator<L, B>
where
    L: Iterator,
    B: Bitmap<L::Item>,
    L::Item: Clone + Copy,
{
    pub linear_iterator: L,
    pub bitmap: B,
}

impl<L, B> Iterator for IntermediateIterator<L, B>
where
    L: Iterator,
    B: Bitmap<L::Item>,
    L::Item: Clone + Copy,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let pos = self.linear_iterator.next()?;
            if self.bitmap.active(pos) {
                return Some(pos);
            }
        }
    }
}
