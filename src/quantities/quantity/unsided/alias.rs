///! The number of atoms of a token, obtained by normalizing raw atoms.
///!
///! Every token is normalized from K decimal places to 6 decimal places.
///! * USDC (6 decimal places): no change
///! * ETH (18 decimal places): Normalized from 18 -> 6 decimal places
///!
///! This allows us to use u64 instead of U256 to represent atoms.
///! These atoms are grouped into lots.
///!
///! Important- Decimal places must be in the range [6, 18]
///!
///! * Raw atoms will be undefined if decimal places are less than 6
///!
///! * Since raw atoms are internally capped to 128 bits, decimal places more than
///! 19 can overflow this value.
///!
///! * We remove 12 places for 18 decimals. To convert into Raw atoms again,
///! we need floor (log2 (u64::MAX * 10^12)) + 1 = 104 bits to represent u64::MAX atoms.
///! 104 bits fit within u128.
///!
///! # Math
///!
///! atoms = |raw atoms / 10^(K - 6)|
///! - For USDC = raw atoms / 10^0 = raw atoms
///! - For eth = raw atoms / 10^(18 - 6) = raw atoms / 10^12
///
use crate::quantities::{Quantity, SidedDim, N1, P1, Z0};

pub type Unsided<L, U, A, I> = Quantity<SidedDim<L, U, A>, I>;
pub type UnsidedAtoms = Unsided<Z0, Z0, P1, u64>;
pub type DeltaAtoms = Unsided<Z0, Z0, P1, i64>;
pub type DeltaLots = Unsided<P1, Z0, Z0, i64>;

pub type UnsidedDeltaAtomsPerUnit = Unsided<Z0, N1, P1, i64>;
