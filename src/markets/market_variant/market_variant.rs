///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either dynamic or custom
use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::{ArgsBuffer, Decodable},
    instructions::ix_take,
    markets::{HardcodedMarketList, MarketHeader, MarketWithKey, MarketWithKeyRef},
    settlement::{
        global_delta::{ERC20Delta, ERC20MakerDeltas, ERC20SenderDeltas, UnsidedMakerDelta},
        Delta,
    },
    state::{DynamicMarketHasher, DynamicMarketKey, MarketState, SlotKey, SlotState},
    token::{CustomToken, TokenMarker, ERC20, ETH},
    types::{Address, Base, Pair, Quote, TupleReader},
};

#[derive(Clone, Copy, Default)]
pub struct Hardcoded;

#[derive(Clone, Copy, Default)]
pub struct Dynamic;

pub trait MarketVariant: Clone + Copy {
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;

    /// The token index type for this market variant
    ///
    /// Hardcoded variant uses hardcoded token index whereas the dynamic
    /// variant uses an enum of hardcoded and custom token index
    type TokenIndex: Clone + Copy;

    /// Key to read market state slot
    type MarketKey<B: TokenMarker, Q: TokenMarker>: SlotKey;

    /// The decoded market as read from args
    ///
    /// # Variants
    ///
    /// * Hardcoded: This is simply the MarketIndex, used for looking up the
    /// market from static list.
    ///
    /// * Dynamic: The market params are decoded from args and the key is hashed.
    type DecodedMarket<B: TokenMarker, Q: TokenMarker>;

    /// Get DecodedMarket from args
    fn decode<B, Q>(
        args: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::DecodedMarket<B, Q>, GoblinError>
    where
        B: TokenMarker + Decodable<B::TokenIndex<Dynamic>>,
        Q: TokenMarker + Decodable<Q::TokenIndex<Dynamic>>,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>;

    fn get_market_with_key_ref<'a, B, Q>(
        black_box: &'a Self::DecodedMarket<B, Q>,
    ) -> Result<MarketWithKeyRef<'a, Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        MarketWithKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>;

    fn process<B, Q>(
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker
            + 'static
            + Decodable<B::TokenIndex<Dynamic>>
            + Decodable<B::Deposit>
            + TupleReader<
                <ETH as TokenMarker>::Deposit,
                <ERC20 as TokenMarker>::Deposit,
                (ETH, ERC20),
                Result = <B as TokenMarker>::Deposit,
            >,
        Q: TokenMarker
            + 'static
            + Decodable<Q::TokenIndex<Dynamic>>
            + Decodable<Q::Deposit>
            + TupleReader<
                <ETH as TokenMarker>::Deposit,
                <ERC20 as TokenMarker>::Deposit,
                (ETH, ERC20),
                Result = <Q as TokenMarker>::Deposit,
            >,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
        MarketWithKey<Hardcoded, B, Q>: HardcodedMarketList<B, Q>,
        MarketState<Self, B, Q>: SlotState<Self::MarketKey<B, Q>>,
    {
        let market_header = MarketHeader::decode(&ctx.args, offset, len)?;

        let black_box = Self::decode(&ctx.args, offset, len, custom_erc20_list)?;
        let market_with_key_ref = Self::get_market_with_key_ref(&black_box)?;

        let mut market_state =
            MarketState::<Self, B, Q>::load(market_with_key_ref.key).into_inner();

        if market_header.decode_deposit_amounts {
            let base_deposit = B::decode(&ctx.args, offset, len)?;
            let quote_deposit = Q::decode(&ctx.args, offset, len)?;
            let deposit_pair = Pair::new(base_deposit, quote_deposit);

            delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
        }

        // Take bid and take quote
        if Base::get(&market_header.execute_takes) {
            ix_take::<Self, B, Q, Base>(
                ctx,
                offset,
                len,
                &mut delta.local,
                market_with_key_ref.common_market,
                &mut market_state,
            )?;
        }

        if Quote::get(&market_header.execute_takes) {
            ix_take::<Self, B, Q, Quote>(
                ctx,
                offset,
                len,
                &mut delta.local,
                market_with_key_ref.common_market,
                &mut market_state,
            )?;
        }

        // // TODO commit local delta into global delta

        // Reset local delta for reuse
        delta.local.deposits.reset::<B, Q>();

        Ok(())
    }

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
