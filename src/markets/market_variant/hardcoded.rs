use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, Decodable},
    markets::{Hardcoded, HardcodedMarketIndex, HardcodedMarketList, MarketAndKey, MarketVariant},
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

    type DecodedMarket<B: TokenMarker, Q: TokenMarker> = HardcodedMarketIndex;

    type MarketKey<B: TokenMarker, Q: TokenMarker> = HardcodedMarketKey<B, Q>;

    fn decode<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        _custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
    {
        <HardcodedMarketIndex as Decodable<HardcodedMarketIndex>>::decode(args, offset, len)
    }

    fn market_and_key_ref<'a, B, Q>(
        decoded_market: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<&'a MarketAndKey<Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        MarketAndKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
    {
        MarketAndKey::<Hardcoded, B, Q>::HARDCODED_MARKET_LIST
            .get(decoded_market.0)
            .ok_or(GoblinError::InvalidHardcodedMarket)
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
