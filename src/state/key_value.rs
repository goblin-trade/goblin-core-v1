use crate::state::{Preimage, SlotKey};

/// Slot key and value
pub struct KeyValue<P: Preimage> {
    pub key: SlotKey<P>,
    pub value: P::SlotState,
}
