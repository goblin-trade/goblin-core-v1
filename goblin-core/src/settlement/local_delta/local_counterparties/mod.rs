pub mod local_counterparty;

pub use local_counterparty::*;

use crate::types::{Address, FixedMap};

pub const MAX_COUNTERPARTY: usize = 16;

pub type LocalCounterparties = FixedMap<Address, LocalCounterparty, MAX_COUNTERPARTY>;
