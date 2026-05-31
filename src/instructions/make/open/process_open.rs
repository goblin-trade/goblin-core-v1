use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
        update::Increase,
    },
    goblin_error::GoblinError,
    instructions::{MakeReadables, PosHeader},
    quantities::Ticks,
    settlement::CheckedAdd,
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        Preimage,
    },
    types::StoreReader,
};

pub fn process_open<M, B, Q, In>(
    make_readables: &MakeReadables<M, B, Q>,
    writables: &mut Writables,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    // Can we share code with Increase?
    // Code is similar. Just that instead of reading value from slot we overwrite it
    let Readables {
        msg_sender,
        market_readables,
    } = *make_readables.readables;

    let PosHeader {
        position,
        base_lots,
    } = make_readables.pos_header;

    let resting_order_key = RestingOrderPreimage {
        market_key: market_readables.market_key,
        position,
    }
    .hash();

    resting_order_key.store(&RestingOrder {
        maker: *msg_sender,
        base_lots,
    });

    // Update delta
    let amount = <In::Opposite as LegMatcher>::matching_lots_maker(
        base_lots,
        market_readables.market.tick_size,
        position.into(),
    );

    let sided_make_delta = In::get_leg_mut(&mut writables.local_delta.local_sender_delta);
    let delta = Increase::get_leg_mut(&mut sided_make_delta.make);
    *delta = delta.checked_add(amount).ok_or(GoblinError::Overflow)?;

    Ok(())
}
