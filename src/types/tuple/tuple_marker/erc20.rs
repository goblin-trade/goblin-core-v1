use crate::{quantities::DeltaAtoms, token::ERC20, types::TupleMarker};

impl TupleMarker for ERC20 {
    type Deposit = DeltaAtoms;
}
