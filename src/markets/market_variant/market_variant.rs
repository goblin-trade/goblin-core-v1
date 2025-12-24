///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either dynamic or custom
use crate::{
    goblin_error::GoblinError,
    hostio::HostioContext,
    input_processor::Decodable,
    instructions::ix_take,
    markets::{CommonMarket, HardcodedMarket, HardcodedMarketList, MarketHeader, MarketWithKeyRef},
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
    const DISCRIMINATOR: u8;

    type TokenIndex: Clone + Copy;

    /// Key to read market state slot
    type MarketKey<B: TokenMarker, Q: TokenMarker>: SlotKey;

    type BlackBox<B: TokenMarker, Q: TokenMarker>;

    type Market<B: TokenMarker, Q: TokenMarker>;

    fn get_market_with_key_ref<'a, B, Q>(
        black_box: &'a Self::BlackBox<B, Q>,
    ) -> Result<MarketWithKeyRef<'a, Self, B, Q>, GoblinError>
    where
        B: TokenMarker + 'static,
        Q: TokenMarker + 'static,
        HardcodedMarket<B, Q>: HardcodedMarketList<B, Q>;

    fn process<B, Q>(
        ctx: &HostioContext,
        offset: &mut usize,
        len: usize,
        delta: &mut Delta,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError>
    where
        B: TokenMarker
            + Decodable<B::Deposit>
            + TupleReader<
                <ETH as TokenMarker>::Deposit,
                <ERC20 as TokenMarker>::Deposit,
                (ETH, ERC20),
                Result = <B as TokenMarker>::Deposit,
            >,
        Q: TokenMarker
            + Decodable<Q::Deposit>
            + TupleReader<
                <ETH as TokenMarker>::Deposit,
                <ERC20 as TokenMarker>::Deposit,
                (ETH, ERC20),
                Result = <Q as TokenMarker>::Deposit,
            >,
        Self::Market<B, Q>: Decodable<Self::Market<B, Q>>,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>,
        MarketState<Self, B, Q>: SlotState<Self::MarketKey<B, Q>>,
    {
        let market_header = MarketHeader::decode(&ctx.args, offset, len)?;

        let market = Self::Market::<B, Q>::decode(&ctx.args, offset, len)?;
        let market_key = Self::get_market_key(&market, custom_erc20_list)?;

        // let mut market_state = MarketState::<Self, B, Q>::load(&market_key).into_inner();

        // if market_header.decode_deposit_amounts {
        //     let base_deposit = B::decode(&ctx.args, offset, len)?;
        //     let quote_deposit = Q::decode(&ctx.args, offset, len)?;
        //     let deposit_pair = Pair::new(base_deposit, quote_deposit);

        //     delta.local.deposits.set_deposits::<B, Q>(&deposit_pair);
        // }

        // // Take bid and take quote
        // if Base::get(&market_header.execute_takes) {
        //     ix_take::<Self, B, Q, Base>(
        //         ctx,
        //         offset,
        //         len,
        //         &mut delta.local,
        //         Self::common_market(&market),
        //         &mut market_state,
        //     )?;
        // }

        // if Quote::get(&market_header.execute_takes) {
        //     ix_take::<Self, B, Q, Quote>(
        //         ctx,
        //         offset,
        //         len,
        //         &mut delta.local,
        //         Self::common_market(&market),
        //         &mut market_state,
        //     )?;
        // }

        // // TODO commit local delta into global delta

        // // Reset local delta for reuse
        // delta.local.deposits.reset::<B, Q>();

        Ok(())
    }

    /// Get the market key. This key is used to read market state from slot.
    fn get_market_key<B, Q>(
        market: &Self::Market<B, Q>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self::MarketKey<B, Q>, GoblinError>
    where
        B: TokenMarker,
        Q: TokenMarker,
        DynamicMarketKey<B, Q>: DynamicMarketHasher<B, Q>;

    fn common_market<B, Q>(market: &Self::Market<B, Q>) -> &CommonMarket<Self, B, Q>
    where
        B: TokenMarker,
        Q: TokenMarker;

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
