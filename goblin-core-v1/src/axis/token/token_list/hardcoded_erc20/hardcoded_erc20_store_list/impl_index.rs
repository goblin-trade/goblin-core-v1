use core::ops::Index;

use crate::{
    axis::{
        token_list::HardcodedERC20StoreList, HardcodedCallerIndex, HardcodedERC20,
        HardcodedERC20Index,
    },
    state::{SlotKey, StorePreimage},
};

impl Index<(HardcodedCallerIndex, HardcodedERC20Index)> for HardcodedERC20StoreList {
    type Output = SlotKey<StorePreimage<HardcodedERC20>>;

    fn index(&self, index: (HardcodedCallerIndex, HardcodedERC20Index)) -> &Self::Output {
        &self.inner[index.0.inner][index.1.inner]
    }
}
