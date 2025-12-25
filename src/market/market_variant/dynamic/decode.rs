use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    market::{CommonMarket, Dynamic, LotSizePair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    require,
    token::TokenMarker,
    types::Pair,
};

impl<B, Q> Decodable<Self> for CommonMarket<Dynamic, B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    B::TokenIndex<Dynamic>: Decodable<B::TokenIndex<Dynamic>>,
    Q::TokenIndex<Dynamic>: Decodable<Q::TokenIndex<Dynamic>>,
{
    fn decode(args: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<Self, GoblinError> {
        let base_token_index = B::TokenIndex::<Dynamic>::decode(args, offset, len)?;
        let quote_token_index = Q::TokenIndex::<Dynamic>::decode(args, offset, len)?;

        let token_index_pair = Pair::new(base_token_index, quote_token_index);

        require!(len >= *offset + 3, GoblinError::InvalidPayload);
        let lot_size_pair = *args.decode_ref_unchecked::<LotSizePair>(offset);

        let tick_size = *args.decode_ref_unchecked::<QuoteLotsPerBaseUnitPerTick>(offset);

        Ok(CommonMarket::<Dynamic, B, Q> {
            token_index_pair,
            lot_size_pair,
            tick_size,
        })
    }
}
