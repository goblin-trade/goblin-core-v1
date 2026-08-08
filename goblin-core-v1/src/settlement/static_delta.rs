use crate::settlement::{
    global_delta::GlobalDelta, local_delta::local_take::TakeCounterparties, ConstZero,
};

static mut STATIC_DELTA: StaticDelta = StaticDelta::ZEROED;

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

impl ConstZero for StaticDelta {
    const ZEROED: Self = Self {
        global: GlobalDelta::ZEROED,
        take_counterparties: TakeCounterparties::ZEROED,
    };
}

impl StaticDelta {
    pub fn get() -> &'static mut Self {
        unsafe { &mut STATIC_DELTA }
    }
}
