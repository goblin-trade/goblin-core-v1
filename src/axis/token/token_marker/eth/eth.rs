use crate::{
    axis::{
        token::{
            token_index::{CustomERC20List, TokenData},
            token_marker::{eth::ETHDelta, TokenMarker},
            ETHStub, ETH,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    input_processor::ETHTransfers,
    quantities::{ETHAtoms, UnsidedAtoms, UnsidedDeltaAtomsPerLot},
    types::Address,
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex = ETHStub;
    type LocalDeposit = ETHStub;
    type GlobalDeposit = ETHStub;

    type TokenMsgTransfer = ETHTransfers;

    type SenderDelta = ETHDelta;
    type DataList = ETHStub;

    fn token_index_data_iter(
        _custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)> {
        core::iter::once(TokenData::ETH_STUB_PAIR)
    }

    fn get_global_deposit(
        _local_deposit: Self::LocalDeposit,
        _atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        ETHStub
    }

    // fn get_global_token_delta(
    //     _token_index: Self::TokenIndex,
    //     global_sender: &mut GlobalSender,
    // ) -> &mut TokenDelta<Self> {
    //     Self::get_leg_mut(global_sender)
    // }

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        _token_address: &Self::TokenAddress,
        _decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let amount = ETHAtoms::try_from(deposit)?;
        UM::update_eth(trader, &amount)
    }

    ////////////////////

    type TokenAddress = ETHStub;

    type HardcodedDecimals = ETHStub;
    type HostioDecimals = ETHStub;

    // Store 18 decimals in ETHStore for symmetry?
    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        _token_index: Self::TokenIndex,
        _custom_erc20_list: CustomERC20List,
    ) -> Self::TokenAddress {
        ETHStub
    }

    fn get_hostio_decimals(
        _address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        Ok(ETHStub)
    }
}
