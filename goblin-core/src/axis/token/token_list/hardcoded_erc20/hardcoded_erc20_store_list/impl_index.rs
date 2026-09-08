use core::ops::Index;

use crate::{
    axis::{
        caller::HardcodedCaller,
        token::{token_list::HardcodedERC20StoreList, HardcodedERC20},
    },
    state::{SlotKey, StoreKeyIndex, StorePreimage},
};

impl Index<&StoreKeyIndex<HardcodedCaller, HardcodedERC20>> for HardcodedERC20StoreList {
    type Output = SlotKey<StorePreimage<HardcodedERC20>>;

    fn index(&self, index: &StoreKeyIndex<HardcodedCaller, HardcodedERC20>) -> &Self::Output {
        &self.inner[index.caller_locator.inner][index.token_index.inner]
    }
}
