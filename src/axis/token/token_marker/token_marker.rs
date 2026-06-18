use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::LotSizePair,
        token::token_marker::custom_erc20::custom_erc20_data::CustomERC20Data,
    },
    goblin_error::GoblinError,
    input_processor::Decodable,
    settlement::{
        global_delta::{GlobalMakerDeltas, MakerDeltaMap, SenderTokenStore},
        local_delta::DepositTriple,
        CheckedAdd, ConstZero, Delta, SidedSenderDeltaV2, UnsideDelta, UnsidedSenderDeltaV2,
    },
    types::StoreReader,
};

/// Marker class for 'Token'. We have 3 variants- ETH, HardcodedERC20 and CustomERC20
pub trait TokenMarker:
    Clone
    + Copy
    + PartialEq
    + 'static
    + StoreReader<GlobalMakerDeltas, Result = MakerDeltaMap<Self>>
    + StoreReader<DepositTriple, Result = Self::Deposit>
{
    const DISCRIMINATOR: u8;

    /// Index to lookup token address
    type TokenIndex: Clone + Copy + Decodable + ConstZero + PartialEq;

    /// Token address
    type Address: Clone + Copy + Sized + Default;

    /// Data type representing pending deposit amount
    type Deposit: Clone + Copy + Default + Decodable + ConstZero + CheckedAdd;

    fn token_index_to_address(
        token_index: Self::TokenIndex,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<Self::Address, GoblinError>;

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
        SidedSenderDeltaV2<In>: UnsideDelta<In, Unsided = UnsidedSenderDeltaV2>,
    {
        let leg_deposit = In::get_leg(&delta.local.deposits);
        let deposit = Self::get(leg_deposit);

        let local_sender_delta = In::get_leg(&delta.local.local_sender_delta);
        let unsided_sender_delta = local_sender_delta.unside(lot_size_pair);

        let global_delta = Self::get_global_delta::<In>(token_index, delta)?;

        // 1. Add deposit
        global_delta.deposit_due = global_delta
            .deposit_due
            .checked_add(deposit)
            .ok_or(GoblinError::Overflow)?;

        // 2. Add sender delta
        global_delta.unsided_sender_delta = global_delta
            .unsided_sender_delta
            .checked_add(unsided_sender_delta)
            .ok_or(GoblinError::Overflow)?;

        Ok(())
    }
}
