use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, leg_quantities::LegQuantities},
        token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    quantities::UnsidedDeltaAtomsPerLot,
    settlement::{
        global_delta::{GlobalMakerDeltas, MakerDeltaMap, SenderTokenStore},
        global_delta_v3::{CounterpartyMap, CounterpartyTriple, GlobalSender, TokenDeltaV3},
        local_delta::DepositTriple,
        local_delta_v3::DepositTripleV3,
        CheckedOps, ConstZero, Delta,
    },
    types::{Address, StoreReader},
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker:
    'static
    + Clone
    + Copy
    + PartialEq
    + StoreReader<GlobalMakerDeltas, Result = MakerDeltaMap<Self>>
    + StoreReader<DepositTriple, Result = Self::GlobalDeposit>
    + StoreReader<DepositTripleV3, Result = Self::LocalDeposit>
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

    // TODO remove, replaced by get_token_delta_v3()
    fn get_global_delta<In>(
        token_index: Self::TokenIndex,
        delta: &mut Delta,
    ) -> Result<&mut SenderTokenStore<Self>, GoblinError>
    where
        In: LegMatcher;

    fn get_token_delta_v3(
        token_index: Self::TokenIndex,
        global_sender: &mut GlobalSender,
    ) -> &mut TokenDeltaV3<Self>;

    fn settle_deposit(
        deposit: Self::GlobalDeposit,
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
        msg_sender: &Address,
    ) -> Result<(), GoblinError>;
}
