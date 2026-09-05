use crate::{
    axis::{ETHStub, ETH, HARDCODED_CALLER_LIST},
    state::{SlotKey, StorePreimage},
};

pub const ETH_STORE_HASH_LIST: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()] =
    build_eth_store_hashes();

const fn build_eth_store_hashes() -> [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()]
{
    let mut hashes = [SlotKey::default(); HARDCODED_CALLER_LIST.inner.len()];
    let mut i = 0;
    while i < hashes.len() {
        hashes[i] = StorePreimage {
            trader: HARDCODED_CALLER_LIST.inner[i],
            token_address: ETHStub,
        }
        .const_hash();
        i += 1;
    }
    hashes
}

// TODO impl Index<HardcodedCallerIndex>
// pub const ETH_STORE_HASH_LIST: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()] = [
//     StorePreimage {
//         trader: HARDCODED_CALLER_LIST.inner[0],
//         token_address: ETHStub,
//     }
//     .const_hash(),
//     StorePreimage {
//         trader: HARDCODED_CALLER_LIST.inner[1],
//         token_address: ETHStub,
//     }
//     .const_hash(),
// ];
