use crate::goblin_error::GoblinError;

pub enum SelfTradeBehavior {
    Abort,
    CancelProvide,
    DecrementTake,
}

impl TryFrom<u8> for SelfTradeBehavior {
    type Error = GoblinError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SelfTradeBehavior::Abort),
            1 => Ok(SelfTradeBehavior::CancelProvide),
            2 => Ok(SelfTradeBehavior::DecrementTake),
            _ => Err(GoblinError::InvalidPayload),
        }
    }
}
