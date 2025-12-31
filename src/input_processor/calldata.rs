use core::mem::MaybeUninit;

use crate::{hostio::hostio_unsafe, input_processor::ArgsBuffer, types::Address};

pub struct Calldata {
    pub args: ArgsBuffer,
    pub msg_sender: Address,
}

impl Calldata {
    pub fn new() -> Self {
        // Allocate uninitialized stack memory
        let mut args = MaybeUninit::<ArgsBuffer>::uninit();
        let mut msg_sender = MaybeUninit::<Address>::uninit();

        unsafe {
            hostio_unsafe::read_args(args.as_mut_ptr() as *mut u8);
            hostio_unsafe::msg_sender(msg_sender.as_mut_ptr() as *mut u8);

            Self {
                args: args.assume_init(),
                msg_sender: msg_sender.assume_init(),
            }
        }
    }
}
