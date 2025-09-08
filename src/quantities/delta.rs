use core::ops::{Add, Sub};

use crate::{
    define_custom_type, define_delta_operations, define_inter_type_operations,
    goblin_error::GoblinError,
    matching::MatchResult,
    quantities::{BaseAtomsPerBaseLot, BaseLots, QuoteAtomsPerQuoteLot, QuoteLots},
    types::SideMarker,
};

use super::Atoms;

// Delta for atoms
define_custom_type!(AtomsDelta<i64>);
define_delta_operations!(AtomsDelta<i64>, Atoms<u64>);

// Delta for lots
define_custom_type!(BaseLotsDelta<i64>);
define_custom_type!(QuoteLotsDelta<i64>);
define_delta_operations!(BaseLotsDelta<i64>, BaseLots<u64>);
define_delta_operations!(QuoteLotsDelta<i64>, QuoteLots<u64>);

define_inter_type_operations!(
    BaseAtomsPerBaseLot<u64>,
    BaseLotsDelta<i64>,
    AtomsDelta<i64>
);
define_inter_type_operations!(
    QuoteAtomsPerQuoteLot<u64>,
    QuoteLotsDelta<i64>,
    AtomsDelta<i64>
);

impl AtomsDelta {
    pub fn abs(&self) -> Atoms {
        Atoms(self.0.abs() as u64)
    }
}
