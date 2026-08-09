use crate::settlement::ConstDefault;

pub type Address = [u8; 20];

impl ConstDefault for Address {
    const DEFAULT: Self = [0u8; 20];
}
