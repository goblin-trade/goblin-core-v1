use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::LotSizePair,
        token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    settlement::{
        global_delta::{GlobalMakerDeltas, MakerDeltaMap, SenderTokenStore},
        local_delta::Deposits,
        ConstZero, Delta, SidedSenderDeltaV2, UnsideDelta, UnsidedSenderDeltaV2,
    },
    types::StoreReader,
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker:
    Clone + Copy + PartialEq + 'static + StoreReader<GlobalMakerDeltas, Result = MakerDeltaMap<Self>>
{
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default + Decodable + ConstZero;

    // type Delta;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

    /// Save deposit amount in deposit store
    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher;

    fn get_global_delta<In>(
        token_index: Self::TokenIndex,
        delta: &mut Delta,
    ) -> Result<&mut SenderTokenStore<Self>, GoblinError>
    where
        In: LegMatcher;

    fn commit_sender_delta<In>(
        token_index: Self::TokenIndex,
        lot_size_pair: &LotSizePair,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>;
}
