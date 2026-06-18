use crate::{quantities::DeltaAtoms, settlement::ConstZero};

impl ConstZero for DeltaAtoms {
    const ZEROED: Self = DeltaAtoms { inner: 0 };
}
