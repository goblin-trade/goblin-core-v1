use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{CommonMarket, DynamicMarket, LotSizePair, TokenIndexPair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    require,
    token::{DynamicIndex, TokenMarker},
};

impl<B, Q> Decodable<DynamicMarket<B, Q>> for DynamicMarket<B, Q>
where
    B: TokenMarker + Decodable<B::TokenIndex<DynamicIndex>>,
    Q: TokenMarker + Decodable<Q::TokenIndex<DynamicIndex>>,
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<DynamicMarket<B, Q>, GoblinError> {
        let base_token_index = B::decode(args, offset, len)?;
        let quote_token_index = Q::decode(args, offset, len)?;

        let token_index_pair = TokenIndexPair::new(base_token_index, quote_token_index);

        require!(len >= *offset + 3, GoblinError::InvalidPayload);
        let lot_size_pair = *args.decode_ref_unchecked::<LotSizePair>(offset);

        let tick_size = *args.decode_ref_unchecked::<QuoteLotsPerBaseUnitPerTick>(offset);

        Ok(DynamicMarket::<B, Q> {
            common: CommonMarket::<DynamicIndex, B, Q> {
                token_index_pair,
                lot_size_pair,
                tick_size,
            },
        })
    }
}
