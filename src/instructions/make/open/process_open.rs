use crate::{
    axis::{
        leg::{leg_matcher::LegMatcher, Base},
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    instructions::{MakeMutables, PosHeader},
    quantities::Ticks,
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        Preimage,
    },
    types::{Address, StoreReader},
};

impl<'a> MakeMutables<'a> {
    pub fn process_open<M, B, Q, In>(
        &mut self,
        msg_sender: &Address,
        market_and_key: &MarketAndKey<M, B, Q>,
        PosHeader {
            position,
            base_lots,
        }: PosHeader,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
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

        self.local_delta
            .local_sender_delta
            .add_resting_order_deposit::<In>(
                base_lots,
                base_lot_size,
                market_and_key.market.tick_size,
                price,
            )
    }
}
