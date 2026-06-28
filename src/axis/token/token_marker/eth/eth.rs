use crate::{
    axis::token::{
        token_marker::{
            custom_erc20::custom_erc20_data::CustomERC20Data, eth::ETHStub, TokenMarker,
        },
        ETH,
    },
    goblin_error::GoblinError,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::global_delta::{GlobalSender, TokenDelta},
    types::{Address, StoreReader},
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ETHStub;
    type Address = ETHStub;

    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    fn token_index_to_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError> {
        Ok(ETHStub)
    }

    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    fn get_global_token_delta(
        _token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self> {
        Self::get_leg_mut(global_sender)
    }

    fn settle_deposit(
        _deposit: Self::GlobalDeposit,
        _token_index: Self::TokenIndex,
        _custom_erc20_list: &[CustomERC20Data],
        _msg_sender: &Address,
    ) -> Result<(), GoblinError> {
        Ok(())
    }
}
