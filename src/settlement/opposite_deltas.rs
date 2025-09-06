use core::mem::MaybeUninit;

use crate::{quantities::Atoms, types::Address};

pub const MAX_OPPOSITE_DELTAS: usize = 15;

/// Deltas of matched makers
/// For side bid, the maker is of side ask.
/// Maker loses base (unlock) and gains quote (free)
///
/// Similarly for side ask
/// Maker gains base (free) and loses quote (locked)
///
/// No need to use signed deltas. Just use Atoms, then add or subtract accordingly
#[derive(Default)]
pub struct OppositeDeltas {
    pub eth: OppositeEthDeltas,
    pub erc20: OppositeERC20Deltas,
}

pub struct OppositeEthDeltas {
    inner: [MaybeUninit<OppositeEthDelta>; MAX_OPPOSITE_DELTAS],
    pub len: usize,
}

pub struct OppositeERC20Deltas {
    inner: [MaybeUninit<OppositeERC20Delta>; MAX_OPPOSITE_DELTAS],
    pub len: usize,
}

impl Default for OppositeEthDeltas {
    fn default() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; MAX_OPPOSITE_DELTAS],
            len: 0,
        }
    }
}

impl Default for OppositeERC20Deltas {
    fn default() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; MAX_OPPOSITE_DELTAS],
            len: 0,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct OppositeEthDelta {
    pub trader: Address,

    pub freed_on_match: Atoms,

    pub unlocked_unlocked_on_match: Atoms,
}

#[derive(Default, Clone, Copy)]
pub struct OppositeERC20Delta {
    pub trader: Address,

    pub token: Address,

    pub freed_on_match: Atoms,

    pub unlocked_on_match: Atoms,
}
