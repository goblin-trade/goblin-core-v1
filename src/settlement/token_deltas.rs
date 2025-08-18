use crate::{
    goblin_error::GoblinError,
    quantities::{Atoms, AtomsDelta},
    settlement::{DeltaAccumulator, ERC20DeltaInput, ERC20DeltaList, EthDelta},
    tokens::TokenIndex,
    types::Address,
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
            erc20_delta_list: ERC20DeltaList::init(indexed_erc20_delta_list)?,
        })
    }

    pub fn add_consumed_amount(
        &mut self,
        token_index: TokenIndex,
        consumed: AtomsDelta,
    ) -> Result<(), GoblinError> {
        if token_index == TokenIndex::ETH {
            self.eth_delta.add_consumed_amount(consumed)
        } else {
            let erc20_delta_item = self.erc20_delta_list.get_or_insert(token_index)?;
            erc20_delta_item.add_consumed_amount(consumed)
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
            let erc20_delta_item = self.erc20_delta_list.get_or_insert(token_index)?;
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

        for erc20_delta in self.erc20_delta_list.iter_mut() {
            erc20_delta.settle(
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
