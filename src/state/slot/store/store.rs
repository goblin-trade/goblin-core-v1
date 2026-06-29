use crate::{
    axis::token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, ETH},
    quantities::UnsidedAtoms,
    state::{Preimage, SlotState},
    types::Address,
};

#[repr(C)]
pub struct Store<T: TokenMarker> {
    pub atoms_locked: UnsidedAtoms,
    pub atoms_free: UnsidedAtoms,
    pub decimals: T::StoredDecimals,
    _padding: T::StoredPadding,
}

unsafe impl<T: TokenMarker> SlotState for Store<T> {}
const _: () = <Store<ETH> as SlotState>::_ASSERT;
const _: () = <Store<HardcodedERC20> as SlotState>::_ASSERT;
const _: () = <Store<CustomERC20> as SlotState>::_ASSERT;
