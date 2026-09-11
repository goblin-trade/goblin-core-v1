use keccak_const::Keccak256;

use crate::state::{Preimage, PreimageSerializer, SlotKey};

pub const trait ConstPreimage: Preimage {
    fn const_hash(&self) -> SlotKey<Self> {
        let buffer = PreimageSerializer::new(*self);
        let bytes = buffer.serialize();
        let hash = Keccak256::new().update(bytes).finalize();
        SlotKey::<Self>::new(hash)
    }
}
