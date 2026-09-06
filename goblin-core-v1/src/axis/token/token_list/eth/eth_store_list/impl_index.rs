use core::ops::Index;

use crate::{
    axis::{token_list::ETHStoreList, HardcodedCaller, ETH},
    state::{SlotKey, StoreKeyIndex, StorePreimage},
};

impl Index<&StoreKeyIndex<HardcodedCaller, ETH>> for ETHStoreList {
    type Output = SlotKey<StorePreimage<ETH>>;

    fn index(&self, index: &StoreKeyIndex<HardcodedCaller, ETH>) -> &Self::Output {
        &self.inner[index.caller_index.inner]
    }
}
