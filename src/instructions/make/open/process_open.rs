use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    quantities::{BaseLots, Position, Ticks},
    settlement::local_delta::LocalSenderDelta,
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        Preimage,
    },
    types::{Address, StoreReader},
};

pub fn process_open<M, B, Q, In>(
    msg_sender: &Address,
    local_sender_delta: &mut LocalSenderDelta,
    market_and_key: &MarketAndKey<M, B, Q>,
    position_2: Position,
    base_lots: BaseLots,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let resting_order_key = RestingOrderPreimage {
        market_key: market_and_key.market_key,
        position: position_2,
    }
    .hash();

    resting_order_key.store(&RestingOrder {
        maker: *msg_sender,
        base_lots,
    });

    // Update delta
    let base_lot_size = Base::get(&market_and_key.market.lot_size_pair);
    let price = Ticks::from(position_2);

    local_sender_delta.add_resting_order_deposit::<In>(
        base_lots,
        base_lot_size,
        market_and_key.market.tick_size,
        price,
    )
}
