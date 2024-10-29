use stylus_sdk::alloy_primitives::FixedBytes;

use crate::{
    quantities::{BaseLots, Ticks, WrapperU64},
    state::{order::order_id::OrderId, RestingOrderIndex},
};

pub struct ReduceOrderPacket {
    // ID of order to reduce
    pub order_id: OrderId,

    // Reduce at most these many lots. Pass u64::MAX to close
    pub lots_to_remove: BaseLots,

    // Whether to revert the entire TX if reduction fails for this order
    pub revert_if_fail: bool,
}

impl From<&FixedBytes<17>> for ReduceOrderPacket {
    fn from(bytes: &FixedBytes<17>) -> Self {
        ReduceOrderPacket {
            order_id: OrderId {
                price_in_ticks: Ticks::new(u64::from_be_bytes(bytes[0..8].try_into().unwrap())),
                resting_order_index: RestingOrderIndex::new(bytes[8]),
            },
            lots_to_remove: BaseLots::new(u64::from_be_bytes(bytes[9..16].try_into().unwrap())),
            revert_if_fail: (bytes[16] & 0b0000_0001) != 0,
        }
    }
}
