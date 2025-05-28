use crate::{input_processor::PayloadBuffer, quantities::RawAtoms, types::Address};

use super::{hostio, HostioBuffer};

pub unsafe fn hostio_read_args() -> HostioBuffer<PayloadBuffer> {
    HostioBuffer::<PayloadBuffer>::new(|ptr| hostio::read_args(ptr))
}

pub unsafe fn hostio_msg_sender() -> HostioBuffer<Address> {
    HostioBuffer::<Address>::new(|ptr| hostio::msg_sender(ptr))
}

pub unsafe fn hostio_msg_value() -> HostioBuffer<RawAtoms> {
    HostioBuffer::<RawAtoms>::new(|f| hostio::msg_value(f))
}
