use crate::{
    axis::{
        caller::HARDCODED_CALLER_LIST,
        token::{HardcodedERC20, HARDCODED_ERC20_LIST},
    },
    settlement::ConstDefault,
    state::{SlotKey, StorePreimage},
};

pub struct HardcodedERC20StoreList {
    pub inner: [[SlotKey<StorePreimage<HardcodedERC20>>; HARDCODED_ERC20_LIST.inner.len()];
        HARDCODED_CALLER_LIST.inner.len()],
}

/// Store hashes for [HardcodedCaller, TokenMarker = ERC20]
///
/// # Stable rust limitation
///
/// Const maps, range loops and Index trait can't be used in const in stable rust.
/// Therefore we use low level looping
pub const HARDCODED_ERC20_STORE_LIST: HardcodedERC20StoreList = HardcodedERC20StoreList {
    inner: const {
        let mut list = [[SlotKey::DEFAULT; HARDCODED_ERC20_LIST.inner.len()];
            HARDCODED_CALLER_LIST.inner.len()];

        let mut i = 0;
        while i < HARDCODED_CALLER_LIST.inner.len() {
            let list_inner = &mut list[i];

            let mut j = 0;
            while j < HARDCODED_ERC20_LIST.inner.len() {
                let token_address = HARDCODED_ERC20_LIST.inner[j].address;

                list_inner[j] = StorePreimage {
                    trader: HARDCODED_CALLER_LIST.inner[i],
                    token_address,
                }
                .const_hash();

                j += 1;
            }

            i += 1;
        }
        list
    },
};
