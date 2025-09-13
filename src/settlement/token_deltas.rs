use crate::{
    goblin_error::GoblinError,
    markets::{IndexedMarketV2, LegLotsDelta, MarketLeg},
    quantities::{Atoms, AtomsDelta},
    settlement::{DeltaAccumulator, ERC20DeltaInput, ERC20DeltaList, EthDelta},
    tokens::TokenIndex,
    types::{Address, LegMarker},
};

pub struct TokenDeltas {
    pub eth_delta: EthDelta,
    pub erc20_delta_list: ERC20DeltaList,
}

impl TokenDeltas {
    pub fn new(
        track_msg_value: bool,
        eth_withdrawal_due: Option<&Atoms>,
        indexed_erc20_delta_list: &[ERC20DeltaInput],
    ) -> Result<Self, GoblinError> {
        Ok(TokenDeltas {
            eth_delta: EthDelta::init(track_msg_value, eth_withdrawal_due)?,
            erc20_delta_list: ERC20DeltaList::new(indexed_erc20_delta_list)?,
        })
    }

    fn apply_leg_lots_delta<L: LegMarker>(
        &mut self,
        market_leg: &MarketLeg<L>,
        // lots_per_unit: L::LotsPerUnit,
        leg_lots_delta: &LegLotsDelta<L>,
    ) {
        // Convert to atoms delta
        let atoms_per_lot = L::atoms_per_lot(market_leg.lot_size);
        let leg_atoms_delta = leg_lots_delta.to_atoms_delta(atoms_per_lot);

        // Remove leg from atomsdelta
        // BaseAtomsDelta -> AtomsDelta
        let consumed = AtomsDelta::from(leg_atoms_delta.consumed);
        self.add_consumed_amount(market_leg.token_index, leg_atoms_delta.consumed);
    }

    pub fn apply_market_delta(
        &mut self,
        indexed_market: &IndexedMarketV2,
        lots_delta: &MarketLotsDelta,
    ) -> Result<(), GoblinError> {
        let market_atoms_delta = indexed_market.get_atoms_delta(lots_delta);

        self.add_consumed_amount(
            indexed_market.base_token_index,
            market_atoms_delta.base_atoms_consumed,
        )?;
        self.add_consumed_amount(
            indexed_market.quote_token_index,
            market_atoms_delta.quote_atoms_consumed,
        )?;

        self.add_locked_amount(
            indexed_market.base_token_index,
            market_atoms_delta.base_atoms_locked,
        )?;
        self.add_locked_amount(
            indexed_market.quote_token_index,
            market_atoms_delta.quote_atoms_locked,
        )?;

        Ok(())
    }

    pub fn add_consumed_amount(
        &mut self,
        token_index: TokenIndex,
        consumed: AtomsDelta,
    ) -> Result<(), GoblinError> {
        if token_index == TokenIndex::ETH {
            self.eth_delta.add_consumed_amount(consumed)
        } else {
            let erc20_delta_item_mut = self
                .erc20_delta_list
                .get_or_insert_mut(token_index)
                .ok_or(GoblinError::ERC20DeltaListFull)?;
            erc20_delta_item_mut.add_consumed_amount(consumed)
        }
    }

    pub fn add_locked_amount(
        &mut self,
        token_index: TokenIndex,
        locked: AtomsDelta,
    ) -> Result<(), GoblinError> {
        if token_index == TokenIndex::ETH {
            self.eth_delta.add_locked_amount(locked)
        } else {
            let erc20_delta_item = self
                .erc20_delta_list
                .get_or_insert_mut(token_index)
                .ok_or(GoblinError::ERC20DeltaListFull)?;
            erc20_delta_item.add_locked_amount(locked)
        }
    }

    pub fn settle(
        &mut self,
        msg_sender: &Address,
        recipient: Option<&Address>,
        custom_erc20_list: &[Address],
        withdraw_internally: bool,
        deposit_shortfall: bool,
    ) -> Result<(), GoblinError> {
        self.eth_delta
            .settle(msg_sender, recipient, withdraw_internally)?;

        for (token_index, erc20_delta) in self.erc20_delta_list.iter_mut() {
            erc20_delta.settle(
                *token_index,
                custom_erc20_list,
                msg_sender,
                recipient,
                deposit_shortfall,
                withdraw_internally,
            )?;
        }

        Ok(())
    }
}
