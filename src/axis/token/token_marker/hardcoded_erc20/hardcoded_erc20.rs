use crate::{
    axis::{
        token::{
            token_marker::{
                hardcoded_erc20::HardcodedERC20Deltas, CustomERC20List, TokenData, TokenMarker,
            },
            token_marker::{
                HardcodedERC20Index, HardcodedTokens, HARDCODED_ERC20_COUNT, HARDCODED_TOKENS,
            },
            HardcodedERC20, HardcodedERC20Stub,
        },
        update::UpdateMarker,
    },
    goblin_error::GoblinError,
    quantities::{UnsidedAtoms, UnsidedDeltaAtoms, UnsidedDeltaAtomsPerLot, UnsidedDeltaLots},
    settlement::global_delta::TransferERC20,
    types::Address,
};

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex = HardcodedERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = HardcodedERC20Stub;

    type SenderDeltaList = HardcodedERC20Deltas;
    type DataList<'a> = &'a HardcodedTokens<HARDCODED_ERC20_COUNT>;

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

    ///////////

    type TokenAddress = Address;

    type HardcodedDecimals = u8;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_stored_decimals(
        token_data: &TokenData<Self>,
    ) -> Result<Self::StoredDecimals, GoblinError> {
        Ok(token_data.decimals)
    }

    fn get_data_list<'a>(_custom_erc20_list: CustomERC20List<'a>) -> Self::DataList<'a> {
        &HARDCODED_TOKENS
    }
}
