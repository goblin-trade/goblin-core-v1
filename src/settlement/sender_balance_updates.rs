use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    matching::MatchResult,
    settlement::{CommonDelta, ERC20DeltaList, EthDelta, SenderDelta},
    tokens::DynamicIndex,
    types::{Address, Base, LegMarker, PairAccessor, Quote},
};

pub struct SenderBalanceUpdates {
    pub eth_delta: EthDelta,
    pub erc20_delta_list: ERC20DeltaList,
}

impl SenderBalanceUpdates {
    pub fn new(track_msg_value: bool) -> Result<Self, GoblinError> {
        Ok(SenderBalanceUpdates {
            eth_delta: EthDelta::init(track_msg_value)?,
            erc20_delta_list: ERC20DeltaList::default(),
        })
    }

    fn token_common_delta(
        &mut self,
        token_index: DynamicIndex,
    ) -> Result<&mut CommonDelta, GoblinError> {
        Ok(if token_index == DynamicIndex::ETH {
            &mut self.eth_delta.common_delta
        } else {
            &mut self
                .erc20_delta_list
                .get_or_insert_mut(token_index)
                .ok_or(GoblinError::MakerListFull)?
                .common_delta
        })
    }

    pub fn apply_side_update<In>(
        &mut self,
        indexed_market: &IndexedMarket,
        sender_delta: &SenderDelta,
    ) -> Result<(), GoblinError>
    where
        In: LegMarker
            + PairAccessor<DynamicIndex, DynamicIndex, Result = DynamicIndex>
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            > + PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In>>,
        In::Opposite:
            PairAccessor<MatchResult<Base>, MatchResult<Quote>, Result = MatchResult<In::Opposite>>,
    {
        // Convert delta to Atoms format on Token namespace
        let taker_token_update = sender_delta.to_taker_token_update::<In>(
            indexed_market.lot_size_pair,
            indexed_market.base_lot_size(),
        );

        let token_index = *In::get_leg(&indexed_market.token_index_pair);

        // Update token delta
        let token_common_delta = self.token_common_delta(token_index)?;
        token_common_delta
            .apply_taker_update(&taker_token_update)
            .ok_or(GoblinError::Overflow)
    }

    pub fn apply_updates(
        &mut self,
        indexed_market: &IndexedMarket,
        market_sender_delta: &SenderDelta,
    ) -> Result<(), GoblinError> {
        self.apply_side_update::<Base>(indexed_market, market_sender_delta)?;
        self.apply_side_update::<Quote>(indexed_market, market_sender_delta)?;

        // TODO apply updates on make orders, cancellations
        Ok(())
    }

    pub fn settle(
        &mut self,
        msg_sender: &Address,
        recipient: Option<&Address>,
        custom_erc20_list: &[Address],
        withdraw_internally: bool,
    ) -> Result<(), GoblinError> {
        self.eth_delta
            .settle(msg_sender, recipient, withdraw_internally)?;

        for (token_index, erc20_delta) in self.erc20_delta_list.iter_mut() {
            erc20_delta.settle(
                *token_index,
                custom_erc20_list,
                msg_sender,
                recipient,
                withdraw_internally,
            )?;
        }

        Ok(())
    }
}
