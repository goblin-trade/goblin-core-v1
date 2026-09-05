use crate::{
    axis::{CallerEnum, CallerMarker, TokenMarker},
    match_axes,
    state::{Preimage, SlotKey, Store},
    types::Address,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorePreimage<TM: TokenMarker> {
    pub trader: Address,
    pub token_address: TM::TokenAddress,
}

impl<TM: TokenMarker> StorePreimage<TM> {
    pub fn get_hash(&self, token_index: &TM::TokenIndex) -> SlotKey<Self> {
        let caller_enum = CallerEnum::from(&self.trader);
        match_axes!(CM = caller_enum => {
            Self::get_store_hash::<CM>(self, token_index)
        })
    }

    pub fn get_store_hash<CM: CallerMarker>(&self, token_index: &TM::TokenIndex) -> SlotKey<Self> {
        TM::get_store_hash::<CM>(self, token_index)
    }
}

impl<TM: TokenMarker> Preimage for StorePreimage<TM> {
    const SLOT_DISCRIMINATOR: u8 = 2 + TM::DISCRIMINATOR;
    type SlotState = Store<TM>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axis::{
        caller::{CustomCaller, HardcodedCaller, HARDCODED_CALLER_0, HARDCODED_CALLER_1},
        token::{token_marker::HardcodedERC20Index, ETHStub, HARDCODED_ERC20_LIST},
        CustomERC20, HardcodedERC20, ETH,
    };

    #[test]
    fn test_hardcoded_caller_eth_store_hashes() {
        let eth_p0 = StorePreimage::<ETH> {
            trader: HARDCODED_CALLER_0,
            token_address: ETHStub,
        };
        let eth_p1 = StorePreimage::<ETH> {
            trader: HARDCODED_CALLER_1,
            token_address: ETHStub,
        };

        // Static dispatch with HardcodedCaller
        assert_eq!(
            eth_p0.get_store_hash::<HardcodedCaller>(&ETHStub).hash(),
            eth_p0.hash().hash()
        );
        assert_eq!(
            eth_p1.get_store_hash::<HardcodedCaller>(&ETHStub).hash(),
            eth_p1.hash().hash()
        );

        // Dynamic dispatch via get_hash
        assert_eq!(eth_p0.get_hash(&ETHStub).hash(), eth_p0.hash().hash());
        assert_eq!(eth_p1.get_hash(&ETHStub).hash(), eth_p1.hash().hash());
    }

    #[test]
    fn test_hardcoded_caller_erc20_store_hashes() {
        for (caller_idx, trader) in [HARDCODED_CALLER_0, HARDCODED_CALLER_1]
            .into_iter()
            .enumerate()
        {
            for (token_idx, token_data) in HARDCODED_ERC20_LIST.inner.iter().enumerate() {
                let erc20_preimage = StorePreimage::<HardcodedERC20> {
                    trader,
                    token_address: token_data.address,
                };
                let token_index = HardcodedERC20Index(token_idx);

                // Static dispatch with HardcodedCaller
                assert_eq!(
                    erc20_preimage
                        .get_store_hash::<HardcodedCaller>(&token_index)
                        .hash(),
                    erc20_preimage.hash().hash(),
                    "Failed for caller {caller_idx}, token {token_idx}"
                );

                // Dynamic dispatch via get_hash
                assert_eq!(
                    erc20_preimage.get_hash(&token_index).hash(),
                    erc20_preimage.hash().hash(),
                    "Failed dynamic dispatch for caller {caller_idx}, token {token_idx}"
                );
            }
        }
    }

    #[test]
    fn test_custom_caller_hashes() {
        let custom_trader: Address = [0x99; 20];

        let eth_preimage = StorePreimage::<ETH> {
            trader: custom_trader,
            token_address: ETHStub,
        };
        assert_eq!(
            eth_preimage.get_store_hash::<CustomCaller>(&ETHStub).hash(),
            eth_preimage.hash().hash()
        );
        assert_eq!(
            eth_preimage.get_hash(&ETHStub).hash(),
            eth_preimage.hash().hash()
        );

        let erc20_preimage = StorePreimage::<HardcodedERC20> {
            trader: custom_trader,
            token_address: HARDCODED_ERC20_LIST.inner[0].address,
        };
        let token_index = HardcodedERC20Index(0);
        assert_eq!(
            erc20_preimage
                .get_store_hash::<CustomCaller>(&token_index)
                .hash(),
            erc20_preimage.hash().hash()
        );
        assert_eq!(
            erc20_preimage.get_hash(&token_index).hash(),
            erc20_preimage.hash().hash()
        );
    }

    #[test]
    fn test_custom_erc20_hashes() {
        use crate::axis::token::token_marker::custom_erc20::custom_erc20_index::CustomERC20Index;

        let custom_token_addr: Address = [0xaa; 20];
        let token_index = CustomERC20Index(0);

        let p0 = StorePreimage::<CustomERC20> {
            trader: HARDCODED_CALLER_0,
            token_address: custom_token_addr,
        };
        let p1 = StorePreimage::<CustomERC20> {
            trader: HARDCODED_CALLER_1,
            token_address: custom_token_addr,
        };
        let p_custom = StorePreimage::<CustomERC20> {
            trader: [0x55; 20],
            token_address: custom_token_addr,
        };

        assert_eq!(p0.get_hash(&token_index).hash(), p0.hash().hash());
        assert_eq!(p1.get_hash(&token_index).hash(), p1.hash().hash());
        assert_eq!(
            p_custom.get_hash(&token_index).hash(),
            p_custom.hash().hash()
        );
    }
}
