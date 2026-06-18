use crate::{
    axis::{
        leg::{Base, Quote},
        market::{market_marker::MarketMarker, LotSizePair, TokenIndexPair},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta::{GlobalDelta, MakerDeltaKey},
        local_delta::LocalDelta,
        ConstZero,
    },
    types::StoreReader,
};

/// Global static mut Delta, initially zero filled.
///
/// `static mut` allows us to take advantage of the fact that lienar memory is zero filled.
/// We get an empty starting buffer without the cost of zeroing.
static mut DELTA: Delta = Delta::ZEROED;

pub struct Delta {
    pub global: GlobalDelta,
    pub local: LocalDelta,
}

impl ConstZero for Delta {
    const ZEROED: Self = Self {
        global: GlobalDelta::ZEROED,
        local: LocalDelta::ZEROED,
    };
}

impl Delta {
    pub fn get_static() -> &'static mut Self {
        unsafe { &mut DELTA }
    }

    pub fn commit_local_delta<M, B, Q>(
        &mut self,
        token_index_pair: &TokenIndexPair<B, Q>,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    {
        let base_token_index = Base::get(token_index_pair);
        let quote_token_index = Quote::get(token_index_pair);

        B::commit_sender_delta::<Base>(base_token_index, lot_size_pair, self)?;
        Q::commit_sender_delta::<Quote>(quote_token_index, lot_size_pair, self)?;

        for (maker, delta_pair) in self.local.local_maker_deltas.iter() {
            self.global.maker_deltas.commit::<B, Base>(
                MakerDeltaKey {
                    maker: *maker,
                    token_index: base_token_index,
                },
                delta_pair,
                lot_size_pair,
            )?;

            self.global.maker_deltas.commit::<Q, Quote>(
                MakerDeltaKey {
                    maker: *maker,
                    token_index: quote_token_index,
                },
                delta_pair,
                lot_size_pair,
            )?;
        }

        // Reset local delta for reuse
        self.local.deposits.reset::<B, Q>();
        Ok(())
    }
}
