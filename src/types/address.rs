use crate::settlement::ConstZero;

pub type Address = [u8; 20];

impl ConstZero for Address {
    const ZEROED: Self = [0u8; 20];
}

pub const NATIVE_TOKEN: Address = [0u8; 20];
pub const NATIVE_TOKEN_DECIMALS: u8 = 18;
