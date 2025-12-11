use crate::{hostio::hostio_unsafe, input_processor::ArgsBuffer, types::Address};

pub struct HostioContext {
    pub args: ArgsBuffer,
    pub msg_sender: Address,
}

impl HostioContext {
    pub const fn zero() -> Self {
        Self {
            args: [0u8; 512],
            msg_sender: [0u8; 20],
        }
    }

    pub fn load(&mut self) {
        unsafe {
            hostio_unsafe::read_args(self.args.as_mut_ptr());
            hostio_unsafe::msg_sender(self.msg_sender.as_mut_ptr());
        }
    }
}
