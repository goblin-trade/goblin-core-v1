use crate::{
    axis::token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    goblin_error::GoblinError,
    input_processor::Decodable,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{CounterpartyMap, CounterpartyTriple, GlobalSender, TokenDelta},
        local_delta::DepositTriple,
        CheckedOps, ConstZero,
    },
    types::{Address, StoreReader},
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker:
    'static
    + Clone
    + Copy
    + PartialEq
    + StoreReader<DepositTriple, Result = Self::LocalDeposit>
    + StoreReader<CounterpartyTriple, Result = CounterpartyMap<Self>>
{
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    fn get_global_token_delta(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self>;

    fn settle_deposit(
        deposit: Self::GlobalDeposit,
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
        msg_sender: &Address,
    ) -> Result<(), GoblinError>;
}
