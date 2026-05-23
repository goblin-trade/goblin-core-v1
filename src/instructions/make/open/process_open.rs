use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::PosHeader,
    quantities::Ticks,
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        Preimage,
    },
    types::StoreReader,
};

pub fn process_open<M, B, Q, In>(
    readables: &Readables<M, B, Q>,
    PosHeader {
        position,
        base_lots,
    }: PosHeader,
    writables: &mut Writables,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let Readables {
        msg_sender,
        market_and_key,
    } = *readables;
    let resting_order_key = RestingOrderPreimage {
        market_key: market_and_key.market_key,
        position,
    }
    .hash();

    resting_order_key.store(&RestingOrder {
        maker: *msg_sender,
        base_lots,
    });

    // Update delta
    let base_lot_size = Base::get(&market_and_key.market.lot_size_pair);
    let price = Ticks::from(position);

    writables
        .local_delta
        .local_sender_delta
        .add_resting_order_deposit::<In>(
            base_lots,
            base_lot_size,
            market_and_key.market.tick_size,
            price,
        )
}
