use crate::{
    hostio::hostio_unsafe, input_processor::ArgsBuffer, quantities::RawAtoms, types::Address,
};

pub struct HostioContext {
    pub args: ArgsBuffer,
    pub msg_sender: Address,
    pub msg_value: RawAtoms,
}

impl HostioContext {
    pub const fn new() -> Self {
        Self {
            args: [0u8; 512],
            msg_sender: [0u8; 20],
            msg_value: RawAtoms([0u8; 32]),
        }
    }

    pub fn load_args(&mut self) {
        unsafe {
            hostio_unsafe::read_args(self.args.as_mut_ptr());
        }
    }

    pub fn load_msg_sender(&mut self) {
        unsafe {
            hostio_unsafe::msg_sender(self.msg_sender.as_mut_ptr());
        }
    }

    pub fn load_msg_value(&mut self) {
        unsafe {
            hostio_unsafe::msg_value(self.msg_value.0.as_mut_ptr());
        }
    }
}
