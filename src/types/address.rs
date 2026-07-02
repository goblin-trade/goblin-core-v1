use crate::settlement::ConstZero;

pub type Address = [u8; 20];

impl ConstZero for Address {
    const ZEROED: Self = [0u8; 20];
}
