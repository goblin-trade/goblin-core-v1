pub mod counterparties;
pub mod global_sender;

pub use counterparties::*;
pub use global_sender::*;

use crate::{axis::party::Party, types::Tuple};

pub type GlobalDelta = Tuple<GlobalSender, CounterpartyTriple, Party>;
