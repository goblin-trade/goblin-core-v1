use crate::{
    axis::{
        token::{
            token_index::{
                CustomERC20List, HardcodedERC20Index, TokenData, TokenIndex, HARDCODED_TOKENS,
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
    type TokenIndex = HardcodedERC20Index;

    type LocalDeposit = UnsidedDeltaLots;
    type GlobalDeposit = UnsidedDeltaAtoms;

    type TokenMsgTransfer = HardcodedERC20Stub;

    type SenderDelta = HardcodedERC20Deltas;

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
        token_address: &<Self::TokenIndex as TokenIndex>::TokenAddress,
        decimals: <Self::TokenIndex as TokenIndex>::StoredDecimals,
    ) -> Result<(), GoblinError> {
        TransferERC20::<UM>::new(deposit, trader, token_address, decimals).dispatch()
    }
}
