use crate::{
    axis::token::{
        token_marker::{TokenData, TokenMarker},
        token_msg_transfer::TokenMsgTransfer,
        CustomERC20, HardcodedERC20, ETH,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms},
    settlement::global_delta::TokenDelta,
    state::SlotState,
};

#[repr(C)]
pub struct Store<T: TokenMarker> {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: T::StoredDecimals,
    _padding: T::StoredPadding,
}

impl<T: TokenMarker> Store<T> {
    pub fn update(
        &mut self,
        token_data: &TokenData<T>,
        token_delta: &TokenDelta<T>,
        msg_transfer: &T::TokenMsgTransfer,
    ) -> Result<(), GoblinError> {
        if self.is_empty() {
            self.decimals = T::get_stored_decimals(token_data)?;
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
}

unsafe impl<T: TokenMarker> SlotState for Store<T> {}
const _: () = <Store<ETH> as SlotState>::_ASSERT;
const _: () = <Store<HardcodedERC20> as SlotState>::_ASSERT;
const _: () = <Store<CustomERC20> as SlotState>::_ASSERT;
