use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, LotSizePair},
        token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    quantities::DeltaAtoms,
    settlement::{
        global_delta::{ERC20Delta, GlobalDelta, GlobalSenderDelta},
        local_delta::Deposits,
        ConstZero, Delta, SidedMakeDeltaV2, SidedSenderDeltaV2, SidedTakeDeltaV2, UnsideDelta,
        UnsidedMakeDeltaV2, UnsidedSenderDeltaV2, UnsidedTakeDeltaV2,
    },
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker: Clone + Copy + 'static {
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default + Decodable;

    // type Delta;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

    /// Save deposit amount in deposit store
    fn set_deposit<In>(deposits: &mut Deposits, deposit_amount: Self::Deposit)
    where
        In: LegMatcher;

    fn add_delta<In>(
        delta: &mut Delta,
        token_index: Self::TokenIndex,
        lot_size_pair: &LotSizePair,
        // delta: Self::Delta,
        // token_index: Self::TokenIndex,
        // global_sender_delta: &mut GlobalSenderDelta,
    ) -> Result<(), GoblinError>
    where
        In: LegMatcher,
        // SidedTakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedTakeDeltaV2>,
        // SidedMakeDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedMakeDeltaV2>,
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>;
}
