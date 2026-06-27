use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::LotSizePair,
        token::{token_marker::TokenMarker, CustomERC20, HardcodedERC20, Token, ETH},
    },
    goblin_error::GoblinError,
    settlement::{
        global_delta_v3::{CounterpartyMap, CounterpartyTokenKey},
        local_delta_v3::{delta_lots_pair, DeltaLotsPair},
    },
    types::Triple,
};

pub type Counterparties = Triple<
    CounterpartyMap<ETH>,
    CounterpartyMap<HardcodedERC20>,
    CounterpartyMap<CustomERC20>,
    Token,
>;

impl Counterparties {
    pub fn commit_side<T, In>(
        &mut self,
        counterparty_token_key: &CounterpartyTokenKey<T>,
        delta_lots_pair: &DeltaLotsPair,
        lot_size_pair: &LotSizePair,
    ) -> Result<(), GoblinError>
    where
        T: TokenMarker,
        In: LegMatcher,
    {
        let delta_lots = In::get(delta_lots_pair);

        Ok(())
    }
}
