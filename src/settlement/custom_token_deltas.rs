use core::mem::MaybeUninit;

use crate::{
    tokens::{CustomToken, ERC20TokenTrait, TokenIndex, HARDCODED_TOKENS},
    utils::FixedMap,
};

use super::ERC20Delta;

// Max allowed deltas
//
// The max length of `withdrawal_list` is also 15. If withdrawal_list has 15
// elements we cannot insert more in ERC20DeltaList
pub const MAX_CUSTOM_DELTAS: usize = 8;

pub struct ERC20DeltaMaybe {
    pub init: bool,
    pub inner: MaybeUninit<ERC20Delta>,
}

pub type CustomTokenDeltas = [ERC20DeltaMaybe; MAX_CUSTOM_DELTAS];
pub type HardcodedTokenDeltas = [ERC20DeltaMaybe; HARDCODED_TOKENS.len()];

pub trait DeltaStore {
    fn delta_mut(&mut self, index: TokenIndex<T>) -> &ERC20Delta;
}
// /// New market based design
// /// - We can deposit or withdraw into the same token in multiple market namespaces.
// /// - There is no single deposit or withdraw amount like before. We need to take sum.
// pub type CustomTokenDeltas = FixedMap<TokenIndex<CustomToken>, ERC20Delta, MAX_DELTAS>;
