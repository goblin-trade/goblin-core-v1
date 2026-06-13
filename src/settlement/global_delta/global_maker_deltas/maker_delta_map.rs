use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{global_delta::MakerDeltaKey, ConstZero, UnsidedTakeDeltaV2},
    types::FixedMap,
};

/// Global maker deltas for ETH
pub type MakerDeltaMap<T> = FixedMap<MakerDeltaKey<T>, UnsidedTakeDeltaV2, 16>;

impl<T> ConstZero for MakerDeltaMap<T>
where
    T: TokenMarker,
{
    const ZEROED: Self = Self {
        entries: [(
            MakerDeltaKey {
                maker: [0u8; 20],
                token_index: <T as TokenMarker>::TokenIndex::ZEROED,
            },
            UnsidedTakeDeltaV2::ZEROED,
        ); 16],
        len: 0,
    };
}
