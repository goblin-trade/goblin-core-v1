pub trait SlotKey {
    /// Unique 1 byte discriminator for the slot. We need different discriminators for
    /// different slots sharing the same namespace- eg. FreeAtomState and LockedAtomState
    fn discriminator() -> u8;

    fn to_keccak256(&self) -> [u8; 32];
}

pub trait SlotState<K: SlotKey, S> {
    fn load(key: &K) -> &mut S;
}
