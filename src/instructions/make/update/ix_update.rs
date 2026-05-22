use crate::{
    axis::{
        market::{market_marker::MarketMarker, MarketAndKey},
        token::token_marker::TokenMarker,
        update::UpdateEnum,
    },
    goblin_error::GoblinError,
    instructions::{MakeMutables, PosHeader},
    matching::region::make_region::MakeRegion,
    require,
    types::Address,
};

impl<'a> MakeMutables<'a> {
    pub fn ix_update<M, B, Q>(
        &mut self,
        msg_sender: &Address,
        market_and_key: &MarketAndKey<M, B, Q>,
        pos_header: PosHeader,
        update_enum: UpdateEnum,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let position = pos_header.position;
        let region = MakeRegion::new(&self.market_state.last_positions, position);

        // Cannot update in `Spread` region as it has no orders
        let MakeRegion::In(leg_in) = region else {
            return Err(GoblinError::NoRestingOrder);
        };

        require!(
            self.inner_bitmap_state.index_active(position.into()),
            GoblinError::NoRestingOrder
        );

        self.process_update_cases(msg_sender, market_and_key, pos_header, update_enum, leg_in)
    }
}
