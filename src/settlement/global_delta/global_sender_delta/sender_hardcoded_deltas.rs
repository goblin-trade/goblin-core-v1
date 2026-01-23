use crate::{settlement::global_delta::ERC20Delta, token::HARDCODED_TOKENS};

/// Deltas of hardcoded tokens
pub type SenderHardcodedDeltas = [ERC20Delta; HARDCODED_TOKENS.len()];
