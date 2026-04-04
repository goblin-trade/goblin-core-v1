use crate::quantities::OuterPosV2;

impl OuterPosV2 {
    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }
}
