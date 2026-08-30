pub struct MarketCountsV2 {
    counts: [u8; 11],
    index: usize,
}

impl MarketCountsV2 {
    pub const fn new(counts: [u8; 11], index: usize) -> Self {
        Self { counts, index }
    }

    fn count(&self) -> u8 {
        self.counts[self.index]
    }

    pub fn get_count_and_advance(&mut self) -> u8 {
        let count = self.count();
        self.index += 1;

        count
    }
}
