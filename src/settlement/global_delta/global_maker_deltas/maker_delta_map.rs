use crate::{
    axis::token::token_reader::TokenReader,
    settlement::{global_delta::MakerDeltaKey, ConstZero, UnsidedTakeDeltaV2},
    types::FixedMap,
};

/// Global maker deltas for ETH
pub type MakerDeltaMap<T> = FixedMap<MakerDeltaKey<T>, UnsidedTakeDeltaV2, 16>;

impl<T> ConstZero for MakerDeltaMap<T>
where
    T: TokenReader,
{
    const ZEROED: Self = Self {
        entries: [(
            MakerDeltaKey {
                maker: [0u8; 20],
                token_index: <T as TokenReader>::TokenIndex::ZEROED,
            },
            UnsidedTakeDeltaV2::ZEROED,
        ); 16],
        len: 0,
    };
}
