use crate::{
    axis::token::{
        token_marker::TokenMarker, token_slot_store::TokenSlotStore, CustomERC20, HardcodedERC20,
        ETH,
    },
    quantities::UnsidedAtoms,
    state::{Preimage, SlotState},
    types::Address,
};

#[repr(C)]
pub struct Store<T: TokenSlotStore> {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    // TODO remove?
    //
    // This was an optimization
    // See if decimals is written already, to avoid slot read for decimals
    pub decimals: T::StoredDecimals,
    _padding: T::StoredPadding,
}

unsafe impl<T: TokenMarker> SlotState for Store<T> {}
// const _: () = <Store<ETH> as SlotState>::_ASSERT;
// const _: () = <Store<HardcodedERC20> as SlotState>::_ASSERT;
// const _: () = <Store<CustomERC20> as SlotState>::_ASSERT;
