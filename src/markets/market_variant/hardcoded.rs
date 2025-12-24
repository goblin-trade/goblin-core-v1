use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{
        Hardcoded, HardcodedMarket, HardcodedMarketIndex, HardcodedMarketList, MarketVariant,
        MarketWithKeyRef,
    },
    settlement::global_delta::{
        ERC20Delta, ERC20DeltaList, ERC20MakerDeltaKey, ERC20MakerDeltas, ERC20SenderDeltas,
        UnsidedMakerDelta,
    },
    state::HardcodedMarketKey,
    token::{CustomToken, HardcodedIndex, HardcodedToken, TokenMarker},
    types::{Address, TupleReader},
};

impl MarketVariant for Hardcoded {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = HardcodedIndex;

    type BlackBox<B: TokenMarker, Q: TokenMarker> = HardcodedMarketIndex;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = HardcodedMarketKey<B, Q>;

    fn get_black_box<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        _custom_erc20_list: &[CustomToken],
    ) -> Result<Self::BlackBox<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        <HardcodedMarketIndex as Decodable<HardcodedMarketIndex>>::decode(args, offset, len)
    }

    fn get_market_with_key_ref<'a, B, Q>(
        black_box: &'a Self::BlackBox<B, Q>,
    ) -> Result<MarketWithKeyRef<'a, Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        HardcodedMarket<B, Q>: HardcodedMarketList<B, Q>,
    {
        // TODO map HardcodedMarketIndex to market
        let market_ref = HardcodedMarket::<B, Q>::HARDCODED_MARKET_LIST
            .get(black_box.0)
            .ok_or(GoblinError::InvalidHardcodedMarket)?;

        Ok(MarketWithKeyRef {
            common_market: &market_ref.common,
            key: &market_ref.keccak_hash,
        })
    }

    fn token_sender_delta_mut(
        token_index: Self::TokenIndex,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta {
        HardcodedToken::get_leg_mut(token_sender_deltas).get_delta_mut(token_index)
    }

    fn token_maker_delta_mut(
        token_index: Self::TokenIndex,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta> {
        let key = ERC20MakerDeltaKey { maker, token_index };

        HardcodedToken::get_leg_mut(token_maker_deltas).get_or_insert_mut(key)
    }
}
