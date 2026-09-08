use crate::{
    goblin_error::GoblinError,
    quantities::{RawAtoms, UnsidedAtoms},
    types::Address,
};
use goblin_hostio::hostio_helpers;

pub struct HostioFields {
    pub msg_sender: Address,

    /// ETH atoms deposited via msg_value
    pub msg_value: UnsidedAtoms,
}

impl HostioFields {
    pub fn try_new(read_msg_value: bool) -> Result<Self, GoblinError> {
        let msg_sender = hostio_helpers::msg_sender();

        let msg_value = if read_msg_value {
            let msg_value_raw = RawAtoms::<18>(hostio_helpers::msg_value());
            UnsidedAtoms::try_from(msg_value_raw)?
        } else {
            UnsidedAtoms::default()
        };

        Ok(Self {
            msg_sender,
            msg_value,
        })
    }
}
