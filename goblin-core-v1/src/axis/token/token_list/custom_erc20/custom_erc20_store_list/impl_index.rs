use core::ops::Index;

use crate::{
    axis::{CustomERC20, CustomERC20Index, CustomERC20Stub, HardcodedCallerIndex},
    state::{SlotKey, StorePreimage},
};

impl Index<(HardcodedCallerIndex, CustomERC20Index)> for CustomERC20Stub {
    type Output = SlotKey<StorePreimage<CustomERC20>>;

    fn index(&self, index: (HardcodedCallerIndex, CustomERC20Index)) -> &Self::Output {
        // problem- need custom ERC20 address
        StorePreimage {}
    }
}
