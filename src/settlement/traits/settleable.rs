use crate::{goblin_error::GoblinError, types::Address};

pub trait Settleable {
    fn settle(&self, token_address: &Address, msg_sender: &Address) -> Result<(), GoblinError>;
}
