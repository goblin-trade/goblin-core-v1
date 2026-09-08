use crate::{
    axis::update::{Decrease, Increase},
    settlement::CheckedOps,
};

pub trait UpdateQuantity {
    fn checked_update<K: CheckedOps>(value: K, rhs: K) -> Option<K>;
}

impl UpdateQuantity for Increase {
    fn checked_update<K: CheckedOps>(value: K, rhs: K) -> Option<K> {
        value.checked_add(rhs)
    }
}

impl UpdateQuantity for Decrease {
    fn checked_update<K: CheckedOps>(value: K, rhs: K) -> Option<K> {
        value.checked_sub(rhs)
    }
}
