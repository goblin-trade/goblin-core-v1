use crate::{
    axis::token::token_marker::custom_erc20::custom_erc20_list::CustomERC20List,
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
    type TokenAddress: Clone + Copy + Sized + Default;

    /// Decimals stored in `Store`
    /// Decimals are stored as u8 for ERC20 tokens but not for ETH
    type StoredDecimals: Clone + Copy;

    /// Padding to pad `Store` to 32 bytes
    /// ERC20 store has less padding to accomodate `decimals: u8`
    type StoredPadding: Clone + Copy;

    /// Pending deposit amount in local namespace
    type LocalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    /// Pending deposit amount in global namespace
    type GlobalDeposit: Clone + Copy + Default + Decodable + ConstZero + CheckedOps;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: CustomERC20List,
    ) -> Result<Self::TokenAddress, GoblinError>;

    fn get_global_deposit(
        local_deposit: Self::LocalDeposit,
        atoms_per_lot: UnsidedDeltaAtomsPerLot,
    ) -> Self::GlobalDeposit;

    fn get_global_token_delta(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDelta<Self>;
}
