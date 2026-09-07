use crate::{
    axis::{ETHStub, ETH, HARDCODED_CALLER_LIST},
    settlement::ConstDefault,
    state::{SlotKey, StorePreimage},
};

pub struct ETHStoreList {
    pub inner: [SlotKey<StorePreimage<ETH>>; HARDCODED_CALLER_LIST.inner.len()],
}

// pub const ETH_STORE_LIST: ETHStoreList = ETHStoreList {
//     inner: [
//         // SlotKey::new([
//         //     199, 186, 65, 203, 212, 40, 76, 23, 189, 201, 35, 94, 83, 41, 2, 23, 153, 11, 163, 93,
//         //     25, 222, 163, 42, 1, 76, 240, 114, 31, 122, 80, 70,
//         // ]),
//         SlotKey::new([
//             255, 140, 68, 121, 39, 165, 189, 40, 111, 225, 212, 22, 49, 218, 123, 85, 235, 164,
//             102, 81, 212, 120, 113, 232, 124, 152, 14, 240, 255, 151, 63, 214,
//         ]),
//     ],
// };

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

#[cfg(test)]
mod test {
    use crate::axis::token_list::ETH_STORE_LIST;

    #[test]
    fn test_list() {
        let inner_list = ETH_STORE_LIST.inner.map(|gg| *gg.hash());

        println!(
            "length {}, list {:?}",
            ETH_STORE_LIST.inner.len(),
            inner_list
        );
    }
}
