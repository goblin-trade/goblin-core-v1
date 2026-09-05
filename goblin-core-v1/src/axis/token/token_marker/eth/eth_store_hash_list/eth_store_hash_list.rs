use crate::{
    axis::{ETHStub, ETH, HARDCODED_CALLER_LIST},
    settlement::ConstDefault,
    state::{SlotKey, StorePreimage},
};

pub const ETH_STORE_HASH_LIST: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()] =
    build_eth_store_hashes();

const fn build_eth_store_hashes() -> [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()]
{
    // array::map() is not stable so we need to use raw looping
    let mut hashes = [SlotKey::DEFAULT; HARDCODED_CALLER_LIST.inner.len()];
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
