/// The number of atoms to deposit.
///
/// If the value is positive perform deposit else withdraw to address.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct DeltaAtoms {
    pub inner: i64,
}

impl DeltaAtoms {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}
