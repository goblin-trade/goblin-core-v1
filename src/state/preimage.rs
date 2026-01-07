pub trait Preimage {
    const SLOT_DISCRIMINATOR: u8;

    type SlotState;
}
