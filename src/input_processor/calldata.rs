use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    hostio::{self, hostio_unsafe},
    input_processor::ArgsBuffer,
    require,
    types::Address,
};

pub struct Calldata {
    pub args: ArgsBuffer,
    pub msg_sender: Address,
}

impl Calldata {
    pub fn load() -> Result<Self, GoblinError> {
        let msg_reentrant = hostio::msg_reentrant();
        require!(!msg_reentrant, GoblinError::Reentrant);

        // Allocate uninitialized stack memory
        let mut args = MaybeUninit::<ArgsBuffer>::uninit();
        let mut msg_sender = MaybeUninit::<Address>::uninit();

        unsafe {
            hostio_unsafe::read_args(args.as_mut_ptr() as *mut u8);
            hostio_unsafe::msg_sender(msg_sender.as_mut_ptr() as *mut u8);

            Ok(Self {
                args: args.assume_init(),
                msg_sender: msg_sender.assume_init(),
            })
        }
    }
}
