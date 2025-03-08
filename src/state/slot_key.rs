use core::mem::MaybeUninit;

pub trait SlotKey {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    fn discriminator() -> u8;

    fn to_keccak256(&self) -> [u8; 32];
}

pub trait SlotState<K: SlotKey, S> {
    unsafe fn load<'a>(key: &K, slot: &'a mut MaybeUninit<S>) -> &'a mut S;

    unsafe fn store(&self, key: &K);
}
