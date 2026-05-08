pub unsafe trait SlotState: Sized {
    const _ASSERT: () = assert!(
        core::mem::size_of::<Self>() == 32,
        "SlotState must be exactly 32 bytes"
    );
}

#[macro_export]
macro_rules! impl_checked_slot_state {
    ($t:ty) => {
        unsafe impl $crate::state::SlotState for $t {}
        const _: () = <$t as $crate::state::SlotState>::_ASSERT;
    };
}
