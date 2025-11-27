use crate::{
    markets::MarketVariant,
    settlement::global_delta::{LazyERC20Delta, TokenDeltas},
};

// pub trait DeltaAccessor {
//     fn get_lazy_delta(&mut self, token_index: M) -> &mut LazyERC20Delta;
// }

// impl<M: MarketVariant> DeltaAccessor<M> for TokenDeltas {
//     fn get_lazy_delta(&mut self, token_index: M) -> &mut LazyERC20Delta {
//         todo!()
//     }
// }
