use crate::{
    goblin_error::GoblinError, hostio::hostio_read_args, hostio_buffer::HostioBuffer,
    quantities::Atoms, require, settlement::TokenWithdrawalDue, types::Address,
};

use super::CallHeader;

pub const INPUT_SIZE: usize = 512;
pub type PayloadBuffer = [u8; INPUT_SIZE];

pub struct CallPayload {
    pub len: usize,
    pub offset: usize,
    pub input: HostioBuffer<PayloadBuffer>,
}

impl CallPayload {
    pub fn new(len: usize) -> Self {
        let input = unsafe { hostio_read_args() };

        Self {
            len,
            offset: 0,
            input,
        }
    }

    pub unsafe fn input_ref(&self) -> &PayloadBuffer {
        self.input.as_ref()
    }

    // pub fn validate_length(&self) -> Result<(), GoblinError> {
    //     require!(self.len >= self.offset, GoblinError::InvalidPayload);
    //     Ok(())
    // }

    pub fn advance_offset<T>(&mut self) {
        self.offset += core::mem::size_of::<T>();
    }

    // pub fn advance_offset_for_slice<T>(&mut self, len: usize) -> Result<(), GoblinError> {
    //     self.offset += core::mem::size_of::<T>() * len;
    //     require!(self.len >= self.offset, GoblinError::InvalidPayload);
    //     Ok(())
    // }

    pub fn decode_ref<T>(&mut self) -> &T {
        let start_index = self.offset;
        self.offset += core::mem::size_of::<T>();
        let result = unsafe { &*(self.input_ref()[start_index..self.offset].as_ptr() as *const T) };
        result
    }

    pub fn decode<T: Clone>(&mut self) -> T {
        let result_ref = self.decode_ref::<T>();
        result_ref.clone()
    }

    pub fn decode_slice<T>(&mut self, len: usize) -> Result<&[T], GoblinError> {
        let start_index = self.offset;
        self.offset += core::mem::size_of::<T>();
        // self.advance_offset::<T>()?;

        let result = unsafe {
            core::slice::from_raw_parts(
                self.input_ref()[start_index..self.offset].as_ptr() as *const T,
                len,
            )
        };

        Ok(result)
    }

    pub fn decode_slice_v2<T>(&self, len: usize) -> &[T] {
        unsafe {
            let input_ptr = self.input_ref().as_ptr();
            core::slice::from_raw_parts(input_ptr.add(self.offset) as *const T, len)
        }
    }

    pub fn decode_ref_v2<T>(&self) -> &T {
        let start_index = self.offset - core::mem::size_of::<T>();
        let end_index = self.offset;

        let result = unsafe { &*(self.input_ref()[start_index..end_index].as_ptr() as *const T) };
        result
    }

    pub fn decode_payload(&mut self) -> Result<DecodedPayload, GoblinError> {
        require!(
            self.len >= CallHeader::HEADER_BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let input = unsafe { self.input_ref() };
        let header = CallHeader::init(input);

        require!(
            self.len >= header.payload_size(),
            GoblinError::InvalidPayload
        );

        self.offset += CallHeader::HEADER_BYTE_SIZE;

        // Problem- updating offset is a mutable borrow. However the fields we are reading
        // are read immutably. We cannot touch offset after reading fields.
        //
        // Fix- get rid of offset completely

        let provided_recipient = if header.recipient_provided {
            self.advance_offset::<Address>();
            let provided_recipient = self.decode_ref_v2::<Address>();
            Some(provided_recipient)
        } else {
            None
        };

        // self.advance_offset::<Atoms>();

        // let eth_withdrawal_due = if header.track_eth_delta {
        //     let eth_withdrawal_due = self.decode_ref_v2::<Atoms>();
        //     Some(eth_withdrawal_due)
        // } else {
        //     None
        // };

        // let eth_withdrawal_due = if header.track_eth_delta {
        //     Some(self.decode::<Atoms>())
        // } else {
        //     None
        // };

        let custom_token_list = self.decode_slice_v2::<Address>(header.custom_token_count);
        let token_delta_list = self.decode_slice_v2::<TokenWithdrawalDue>(header.token_delta_count);

        // let custom_token_list = self.decode_slice::<Address>(header.custom_token_count)?;

        // Error- cannot borrow *self as mutable more than once at a time
        // second mutable borrow occurs here
        // let token_delta_list = self.decode_slice::<TokenWithdrawalDue>(header.token_delta_count)?;

        Ok(DecodedPayload {
            header,
            provided_recipient,
            // eth_withdrawal_due,
            // custom_token_list,
            // token_delta_list,
        })
    }
}

pub struct DecodedPayload<'a> {
    pub header: CallHeader,
    pub provided_recipient: Option<&'a Address>,
    // pub eth_withdrawal_due: Option<Atoms>,
    // pub custom_token_list: &'a [Address],
    // pub token_delta_list: &'a [TokenWithdrawalDue],
}
