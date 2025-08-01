pub enum OrderExpiry {
    BlockNumber(u32),
    BlockTime(u32),
}

impl OrderExpiry {
    pub fn new(is_block_number: bool, value: u32) -> Self {
        match is_block_number {
            true => Self::BlockNumber(value),
            false => Self::BlockTime(value),
        }
    }

    pub fn used(&self) -> bool {
        *self.inner() != 0
    }

    fn inner(&self) -> &u32 {
        match self {
            OrderExpiry::BlockNumber(value) => value,
            OrderExpiry::BlockTime(value) => value,
        }
    }
}
