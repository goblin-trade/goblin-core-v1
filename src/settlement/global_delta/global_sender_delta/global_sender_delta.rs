use crate::{
    axis::token::{Token, ETH},
    settlement::{
        global_delta::{SenderCustomDeltas, SenderHardcodedDeltas, SenderTokenStore},
        ConstZero,
    },
    types::Triple,
};

/// The global delta for msg.sender. Stores deltas of ETH and ERC20 tokens.
pub type GlobalSenderDelta =
    Triple<SenderTokenStore<ETH>, SenderHardcodedDeltas, SenderCustomDeltas, Token>;

impl ConstZero for GlobalSenderDelta {
    const ZEROED: Self = Self::new(
        SenderTokenStore::ZEROED,
        SenderHardcodedDeltas::ZEROED,
        SenderCustomDeltas::ZEROED,
    );
}
