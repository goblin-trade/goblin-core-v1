pub enum MakeVariant {
    Increase,
    Decrease,
    OpenBaseIn,
    OpenQuoteIn,
}

impl From<u64> for MakeVariant {
    fn from(value: u64) -> Self {
        match value & 0b11 {
            0 => MakeVariant::Increase,
            1 => MakeVariant::Decrease,
            2 => MakeVariant::OpenBaseIn,
            3 => MakeVariant::OpenQuoteIn,
            _ => unreachable!(),
        }
    }
}
