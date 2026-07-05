use crate::{
    axis::token::{
        token_index::TokenIndex, token_marker::TokenMarker, CustomERC20, HardcodedERC20, ETH,
    },
    quantities::UnsidedAtoms,
    state::SlotState,
};

#[repr(C)]
pub struct Store<T: TokenMarker> {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: <T::TokenIndex as TokenIndex>::StoredDecimals,
    _padding: <T::TokenIndex as TokenIndex>::StoredPadding,
}

unsafe impl<T: TokenMarker> SlotState for Store<T> {}
const _: () = <Store<ETH> as SlotState>::_ASSERT;
const _: () = <Store<HardcodedERC20> as SlotState>::_ASSERT;
const _: () = <Store<CustomERC20> as SlotState>::_ASSERT;
