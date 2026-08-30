use core::cell::Cell;

pub struct MarketCountsV2 {
    counts: [u8; 11],
    index: Cell<usize>,
}

impl MarketCountsV2 {
    pub fn new(counts: [u8; 11]) -> Self {
        Self {
            counts,
            index: Cell::default(),
        }
    }

    fn count(&self) -> u8 {
        self.counts[self.index.get()]
    }

    pub fn get_count_and_advance(&self) -> u8 {
        let count = self.count();
        self.index.set(self.index.get() + 1);

        count
    }
}
