mod impl_index;

use crate::{
    axis::{
        caller::HARDCODED_CALLER_LIST,
        token::{ETH, ETHStub},
    },
    settlement::ConstDefault,
    state::{ConstPreimage, SlotKey, StorePreimage},
};

pub struct ETHStoreList {
    pub inner: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()],
}

/// Store hashes for [HardcodedCaller, TokenMarker = ETH]
///
/// # Stable rust limitation
///
/// Const maps, range loops and Index trait can't be used in const in stable rust.
/// Therefore we use low level looping
pub const ETH_STORE_LIST: ETHStoreList = ETHStoreList {
    inner: const {
        let mut list = [SlotKey::DEFAULT; HARDCODED_CALLER_LIST.inner.len()];
        let mut i = 0;
        while i < list.len() {
            list[i] = StorePreimage {
                trader: HARDCODED_CALLER_LIST.inner[i],
                token_address: ETHStub,
            }
            .const_hash();
            i += 1;
        }
        list
    },
};
