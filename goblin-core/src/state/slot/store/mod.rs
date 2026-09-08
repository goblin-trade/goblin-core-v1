pub mod indexed_preimage;
pub mod store_key_index;
pub mod store_preimage;

pub use indexed_preimage::*;
pub use store_key_index::*;
pub use store_preimage::*;

mod impl_slot_state;

use crate::{
    axis::{
        token::{
            token_marker::{TokenData, TokenMarker},
            token_msg_transfer::TokenMsgTransfer,
        },
        update::{Decrease, Increase},
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms},
    settlement::{
        CheckedOps,
        global_delta::{GlobalCounterparty, TokenDelta},
    },
    state::SlotState,
    types::StoreReader,
};

#[repr(C)]
pub struct Store<TM: TokenMarker> {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: TM::StoredDecimals,
    _padding: TM::StoredPadding,
}

impl<TM: TokenMarker> Store<TM> {
    pub fn update_sender(
        &mut self,
        token_data: &TokenData<TM>,
        token_delta: &TokenDelta<TM>,
        msg_transfer: &TM::TokenMsgTransfer,
    ) -> Result<(), GoblinError> {
        if self.is_empty() {
            self.decimals = TM::get_stored_decimals(token_data)?;
        }

        let atoms_free_delta = UnsidedDeltaAtoms::try_from(self.atoms_free)?
            + token_delta.net_delta()
            + msg_transfer.net_delta()?;

        let atoms_locked_delta = UnsidedDeltaAtoms::try_from(self.atoms_locked)? - token_delta.make;

        // Return error if free atoms > 0 or if we overflow
        self.atoms_free = UnsidedAtoms::try_from(atoms_free_delta)?;
        self.atoms_locked = UnsidedAtoms::try_from(atoms_locked_delta)?;

        Ok(())
    }

    pub fn update_counterparty(
        &mut self,
        counterparty: &GlobalCounterparty,
    ) -> Result<(), GoblinError> {
        self.atoms_locked = self
            .atoms_locked
            .checked_sub(Increase::get(&counterparty.inner))
            .ok_or(GoblinError::Underflow)?;

        self.atoms_free = self
            .atoms_free
            .checked_add(Decrease::get(&counterparty.inner))
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
