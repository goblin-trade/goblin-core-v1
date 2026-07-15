use crate::{
    axis::{
        token::{
            token_marker::TokenData,
            token_marker::{
                custom_erc20::CustomERC20Deltas, CustomERC20Index, CustomERC20List, TokenMarker,
            },
            CustomERC20, CustomERC20Stub,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    hostio::erc20_hostio,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots},
    settlement::global_delta::TransferERC20,
    types::Address,
};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type TokenIndex = CustomERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = CustomERC20Stub;

    type SenderDelta = CustomERC20Deltas;
    type DataList<'a> = CustomERC20List<'a>;

    fn token_index_data_iter(
        custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)> {
        custom_erc20_list
            .inner
            .iter()
            .enumerate()
            .map(|(i, data)| (CustomERC20Index::from(i), *data))
    }

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    fn update<UM: UpdateMarker>(
        deposit: UnsidedAtoms,
        trader: &Address,
        token_address: &Self::TokenAddress,
        decimals: Self::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(deposit, trader, token_address, decimals).dispatch()
    }

    ///////

    type TokenAddress = Address;

    type HardcodedDecimals = CustomERC20Stub;
    type HostioDecimals = u8;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_data_list<'a>(custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a> {
        custom_erc20_list
    }

    fn get_hostio_decimals(
        address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        erc20_hostio::decimals(address)
    }
}
