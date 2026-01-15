use crate::{
    market::{DeltaGetter, Hardcoded},
    settlement::global_delta::{
        ERC20Delta, ERC20DeltaList, ERC20MakerDeltaKey, ERC20MakerDeltas, ERC20SenderDeltas,
        UnsidedMakerDelta,
    },
    token::HardcodedERC20,
    types::{Address, TupleReader},
};

impl DeltaGetter for Hardcoded {
    fn token_sender_delta_mut(
        market_erc20_index: Self::MarketERC20Index,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta {
        HardcodedERC20::get_leg_mut(token_sender_deltas).get_delta_mut(market_erc20_index)
    }

    fn token_maker_delta_mut(
        market_erc20_index: Self::MarketERC20Index,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        let key = ERC20MakerDeltaKey {
            maker,
            erc20_index: market_erc20_index,
        };

        HardcodedERC20::get_leg_mut(token_maker_deltas).get_or_insert_mut(key)
    }
}
