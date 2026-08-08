use goblin_macros::ConstZero;

use crate::goblin_error::GoblinError;

#[derive(Clone, Copy, ConstZero)]
pub struct HardcodedERC20Stub;

impl TryFrom<HardcodedERC20Stub> for u8 {
    type Error = GoblinError;

    fn try_from(_value: HardcodedERC20Stub) -> Result<Self, Self::Error> {
        Err(GoblinError::NoHostioDecimals)
    }
}
