use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    quantities::Atoms,
    settlement::{
        CommonDelta, ERC20DeltaList, ERC20Deposit, ERC20Input, ERC20Withdraw, EthDelta, SenderDelta,
    },
    tokens::TokenIndex,
    types::{Address, Base, LegMarker, PairAccessor, Quote},
};

pub struct SenderBalanceUpdates {
    pub eth_delta: EthDelta,
    pub erc20_delta_list: ERC20DeltaList,
}

impl SenderBalanceUpdates {
    pub fn new(
        track_msg_value: bool,
        eth_withdrawal_due: Option<&Atoms>,
        erc20_deposits_due: &[ERC20Input<ERC20Deposit>],
        erc20_withdrawals_due: &[ERC20Input<ERC20Withdraw>],
    ) -> Result<Self, GoblinError> {
        Ok(SenderBalanceUpdates {
            eth_delta: EthDelta::init(track_msg_value, eth_withdrawal_due)?,
            erc20_delta_list: ERC20DeltaList::new(erc20_deposits_due, erc20_withdrawals_due)?,
        })
    }

    fn token_common_delta(
        &mut self,
        token_index: TokenIndex,
    ) -> Result<&mut CommonDelta, GoblinError> {
        Ok(if token_index == TokenIndex::ETH {
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
            + PairAccessor<TokenIndex, TokenIndex, Result = TokenIndex>
            + PairAccessor<
                <Base as LegMarker>::LotsPerUnit,
                <Quote as LegMarker>::LotsPerUnit,
                Result = In::LotsPerUnit,
            >,
    {
        // Convert delta to Atoms format on Token namespace
        let taker_token_update = sender_delta.to_taker_token_update::<In>(
            indexed_market.lot_size_pair,
            indexed_market.base_lot_size(),
        );

        let token_index = *In::get_leg(&indexed_market.token_index_pair);

        // Update token delta
        let token_common_delta = self.token_common_delta(token_index)?;
        token_common_delta.apply_taker_update(&taker_token_update)
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
