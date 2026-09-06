use core::ops::Index;

use crate::{
    axis::{CustomERC20, CustomERC20Index, CustomERC20Stub, HardcodedCaller, HardcodedCallerIndex},
    state::{IndexedPreimage, SlotKey, StorePreimage},
};

// impl Index<&IndexedPreimage<CustomERC20, HardcodedCaller>> for CustomERC20Stub {
//     type Output = SlotKey<StorePreimage<CustomERC20>>;

//     fn index(&self, index: &IndexedPreimage<CustomERC20, HardcodedCaller>) -> &Self::Output {
//         // problem- function returns reference
//         //
//         // We cannot calculate hash and pass its reference ahead
//         //
//         // Fallback to dedicated method TM::get_store_hash().
//         // We can't use Triple with impl Index
//         StorePreimage {}
//     }
// }
