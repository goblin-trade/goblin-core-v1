use crate::{
    goblin_error::GoblinError,
    markets::IndexedMarket,
    quantities::Atoms,
    settlement::{
        CommonDelta, ERC20DeltaList, ERC20Deposit, ERC20Input, ERC20Withdraw, EthDelta,
        MarketSenderDelta,
    },
    tokens::TokenIndex,
    types::{Address, Base, LegMarker, Quote},
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

    pub fn apply_side_update<In: LegMarker>(
        &mut self,
        indexed_market: &IndexedMarket,
        market_sender_delta: &MarketSenderDelta,
    ) -> Result<(), GoblinError> {
        let base_lot_size = indexed_market.base.lot_size;
        let market_leg = In::market_leg(indexed_market);
        let atoms_per_lot = In::atoms_per_lot(market_leg.lot_size);

        // Free atoms in
        let sender_delta_side = In::sender_delta_ref(market_sender_delta);
        let free_lots_in = In::decode_matching_lots(
            sender_delta_side.pending_update.free_matching_lots_in,
            base_lot_size,
        );
        let free_atoms_in: Atoms = (free_lots_in * atoms_per_lot).into();

        // Locked atoms out
        let sender_delta_opposite =
            <In::Opposite as LegMarker>::sender_delta_ref(market_sender_delta);
        let locked_lots_out = In::decode_matching_lots(
            sender_delta_opposite
                .pending_update
                .locked_matching_lots_out,
            base_lot_size,
        );
        let locked_atoms_out: Atoms = (locked_lots_out * atoms_per_lot).into();

        // Released on self trade
        let self_trade_released_lots =
            In::decode_matching_lots(sender_delta_opposite.released_by_self_trade, base_lot_size);
        let self_trade_released_atoms: Atoms = (self_trade_released_lots * atoms_per_lot).into();

        // Update token delta
        let token_common_delta = self.token_common_delta(market_leg.token_index)?;
        token_common_delta.accumulate_market_delta(
            free_atoms_in,
            locked_atoms_out,
            self_trade_released_atoms,
        )?;

        sender_delta_side.released_by_self_trade;
        Ok(())
    }

    pub fn apply_updates(
        &mut self,
        indexed_market: &IndexedMarket,
        market_sender_delta: &MarketSenderDelta,
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
