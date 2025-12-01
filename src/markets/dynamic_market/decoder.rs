use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{CommonMarket, DynamicMarket, LotSizePair, PairShape},
    quantities::QuoteLotsPerBaseUnitPerTick,
    require,
    tokens::DynamicIndex,
};

impl<P> Decodable<DynamicMarket<P>> for DynamicMarket<P>
where
    P: PairShape + Decodable<P::ResolvedPair<DynamicIndex>>,
{
    fn decode(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<DynamicMarket<P>, GoblinError> {
        let token_index_pair = P::decode(args, offset, len)?;

        require!(len >= *offset + 3, GoblinError::InvalidPayload);
        let lot_size_pair = *args.decode_ref_unchecked::<LotSizePair>(offset);

        let tick_size = *args.decode_ref_unchecked::<QuoteLotsPerBaseUnitPerTick>(offset);

        Ok(DynamicMarket::<P> {
            common: CommonMarket::<DynamicIndex, P> {
                token_index_pair,
                lot_size_pair,
                tick_size,
            },
        })
    }
}
