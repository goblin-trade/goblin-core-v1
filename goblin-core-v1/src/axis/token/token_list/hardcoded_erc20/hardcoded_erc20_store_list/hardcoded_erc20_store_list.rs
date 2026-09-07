use crate::{
    axis::{HardcodedERC20, HARDCODED_CALLER_LIST, HARDCODED_ERC20_LIST},
    settlement::ConstDefault,
    state::{SlotKey, StorePreimage},
};

pub struct HardcodedERC20StoreList {
    pub inner: [[SlotKey<StorePreimage<HardcodedERC20>>; HARDCODED_ERC20_LIST.inner.len()];
        HARDCODED_CALLER_LIST.inner.len()],
}

pub const HARDCODED_ERC20_STORE_LIST: HardcodedERC20StoreList = HardcodedERC20StoreList {
    inner: [
        [
            SlotKey::new([
                168, 26, 225, 76, 200, 167, 54, 114, 7, 206, 40, 194, 64, 111, 120, 141, 197, 34,
                42, 133, 76, 50, 68, 42, 206, 242, 180, 123, 222, 121, 191, 89,
            ]),
            SlotKey::new([
                41, 164, 226, 174, 87, 50, 50, 217, 1, 18, 243, 184, 69, 144, 90, 191, 159, 218,
                39, 203, 222, 142, 252, 97, 12, 81, 167, 173, 41, 211, 212, 173,
            ]),
        ],
        [
            SlotKey::new([
                98, 155, 209, 193, 204, 184, 14, 225, 16, 178, 0, 131, 209, 202, 177, 156, 24, 184,
                109, 218, 234, 158, 184, 20, 248, 126, 216, 224, 112, 48, 112, 101,
            ]),
            SlotKey::new([
                202, 221, 216, 134, 144, 235, 217, 255, 3, 112, 248, 153, 46, 73, 214, 134, 19, 80,
                206, 237, 79, 203, 228, 224, 117, 132, 76, 196, 212, 135, 107, 12,
            ]),
        ],
    ],
};

/// Store hashes for [HardcodedCaller, TokenMarker = ERC20]
///
/// # Stable rust limitation
///
/// Const maps, range loops and Index trait can't be used in const in stable rust.
/// Therefore we use low level looping
// pub const HARDCODED_ERC20_STORE_LIST: HardcodedERC20StoreList = HardcodedERC20StoreList {
//     inner: const {
//         let mut list = [[SlotKey::DEFAULT; HARDCODED_ERC20_LIST.inner.len()];
//             HARDCODED_CALLER_LIST.inner.len()];

//         let mut i = 0;
//         while i < HARDCODED_CALLER_LIST.inner.len() {
//             let list_inner = &mut list[i];

//             let mut j = 0;
//             while j < HARDCODED_ERC20_LIST.inner.len() {
//                 let token_address = HARDCODED_ERC20_LIST.inner[j].address;

//                 list_inner[j] = StorePreimage {
//                     trader: HARDCODED_CALLER_LIST.inner[i],
//                     token_address,
//                 }
//                 .const_hash();

//                 j += 1;
//             }

//             i += 1;
//         }
//         list
//     },
// };

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        let inner_list = HARDCODED_ERC20_STORE_LIST
            .inner
            .map(|outer| outer.map(|inner| *inner.hash()));

        println!(
            "length {}, list {:?}",
            HARDCODED_ERC20_STORE_LIST.inner.len(),
            inner_list
        );
    }
}
