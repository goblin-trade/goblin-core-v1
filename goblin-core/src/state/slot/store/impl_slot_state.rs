use crate::{
    axis::token::{CustomERC20, ETH, HardcodedERC20, token_marker::TokenMarker},
    state::{SlotState, Store},
};

impl<TM: TokenMarker> SlotState for Store<TM> {}

const _: () = <Store<ETH> as SlotState>::_ASSERT;
const _: () = <Store<HardcodedERC20> as SlotState>::_ASSERT;
const _: () = <Store<CustomERC20> as SlotState>::_ASSERT;
