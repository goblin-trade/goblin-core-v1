use crate::{
    axis::{
        token::{
            token_index::{
                CustomERC20List, HardcodedERC20Index, HardcodedTokens, TokenData,
                HARDCODED_ERC20_COUNT, HARDCODED_TOKENS,
            },
            token_marker::{hardcoded_erc20::HardcodedERC20Deltas, TokenMarker},
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

    type SenderDelta = HardcodedERC20Deltas;
    type DataList = HardcodedTokens<HARDCODED_ERC20_COUNT>;

    fn token_index_data_iter(
        _custom_erc20_list: CustomERC20List,
    ) -> impl Iterator<Item = (Self::TokenIndex, TokenData<Self>)> {
        HARDCODED_TOKENS
            .inner
            .iter()
            .enumerate()
            .map(|(index, data)| (HardcodedERC20Index::from(index), *data))
    }

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit {
        local_deposit * atoms_per_lot
    }

    // fn get_global_token_delta(
    //     token_index: Self::TokenIndex,
    //     global_sender: &mut GlobalSender,
    // ) -> &mut TokenDelta<Self> {
    //     let list = Self::get_leg_mut(global_sender);
    //     &mut list[token_index]
    // }

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
    type HostioDecimals = HardcodedERC20Stub;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];

    fn get_address(
        token_index: Self::TokenIndex,
        _custom_erc20_list: CustomERC20List,
    ) -> Self::TokenAddress {
        HARDCODED_TOKENS[token_index].address
    }

    fn get_hostio_decimals(
        _address: &Self::TokenAddress,
    ) -> Result<Self::HostioDecimals, GoblinError> {
        Ok(HardcodedERC20Stub)
    }
}
