///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either dynamic or custom
use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    markets::MarketHeader,
    settlement::{
        global_delta::{ERC20Delta, ERC20MakerDeltas, ERC20SenderDeltas, UnsidedMakerDelta},
        Delta,
    },
    state::SlotKey,
    token::{CustomToken, TokenMarker},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

pub trait MarketVariant: Clone + Copy {
    const DISCRIMINATOR: u8;

    type TokenIndex: Clone + Copy;

    type Market<B: TokenMarker, Q: TokenMarker>;

    /// Key to read market state slot
    type MarketKey<B: TokenMarker, Q: TokenMarker>: SlotKey;

    fn process<B, Q>(
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker + Decodable<B::Deposit>,
        Q: TokenMarker + Decodable<Q::Deposit>,
        Self::Market<B, Q>: Decodable<Self::Market<B, Q>>,
    {
        let market_header = MarketHeader::decode(&ctx.args, offset, len)?;
        let market = Self::Market::<B, Q>::decode(&ctx.args, offset, len)?;

        Ok(())
    }

    fn decode_market<B, Q>(
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
        market_header: &MarketHeader,
    ) -> Result<Self::Market<B, Q>, GoblinError>
    where
        B: TokenMarker + Decodable<B::Deposit>,
        Q: TokenMarker + Decodable<Q::Deposit>,
        Self::Market<B, Q>: Decodable<Self::Market<B, Q>>;

    fn token_sender_delta_mut(
        token_index: Self::TokenIndex,
        token_sender_deltas: &mut ERC20SenderDeltas,
    ) -> &mut ERC20Delta;

    fn token_maker_delta_mut(
        token_index: Self::TokenIndex,
        maker: Address,
        token_maker_deltas: &mut ERC20MakerDeltas,
    ) -> Option<&mut UnsidedMakerDelta>;
}
