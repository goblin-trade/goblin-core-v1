pub unsafe trait SlotState: Sized {
    const _ASSERT: () = assert!(
        core::mem::size_of::<Self>() == 32,
        "SlotState must be exactly 32 bytes"
    );

    fn is_empty(&self) -> bool {
        let bytes: &[u8; 32] = unsafe { &*(self as *const Self as *const [u8; 32]) };
        *bytes == [0u8; 32]
    }
}

#[macro_export]
macro_rules! impl_checked_slot_state {
    ($t:ty) => {
        unsafe impl $crate::state::SlotState for $t {}
        const _: () = <$t as $crate::state::SlotState>::_ASSERT;
    };
}
