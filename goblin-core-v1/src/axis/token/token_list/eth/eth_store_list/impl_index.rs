use core::ops::Index;

use crate::{
    axis::{token_list::ETHStoreList, ETHStub, HardcodedCallerIndex, ETH},
    state::{SlotKey, StorePreimage},
};

impl Index<(HardcodedCallerIndex, ETHStub)> for ETHStoreList {
    type Output = SlotKey<StorePreimage<ETH>>;

    fn index(&self, index: (HardcodedCallerIndex, ETHStub)) -> &Self::Output {
        &self.inner[index.0.inner]
    }
}
