use goblin_macros::ConstDefault;

use crate::settlement::{
    global_delta::GlobalDelta, local_delta::local_take::TakeCounterparties, ConstDefault,
};

static mut STATIC_DELTA: StaticDelta = StaticDelta::DEFAULT;

#[derive(ConstDefault)]
pub struct StaticDelta {
    /// Global delta of sender and counterparties
    pub global: GlobalDelta,

    /// Local buffers of take counterparties
    ///
    /// They are passed to LocalDelta and reset after LocalDelta is committed
    /// into GlobalDelta.
    ///
    /// # Optimization
    ///
    /// We get free zero fills by using mut ref for these buffers. Reset operation
    /// simply sets the count to 0 without overwriting the buffer contents.
    pub take_counterparties: TakeCounterparties,
}

impl StaticDelta {
    pub fn get() -> &'static mut Self {
        unsafe { &mut STATIC_DELTA }
    }
}
