pub mod caller_addresses;
pub mod header;
pub mod header_flags;
pub mod header_refs;

pub use caller_addresses::*;
pub use header::*;
pub use header_flags::*;
pub use header_refs::*;

mod hostio_fields;

use deku::DekuReader;

use crate::{
    axis::{
        leg::Pair,
        party::PartySettle,
        token::{ETHTransfers, MsgTransfers},
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, global_args::hostio_fields::HostioFields},
    market::process_market,
    settlement::StaticDelta,
};

pub struct GlobalArgs<'a> {
    pub flags: HeaderFlags,
    pub header: Header,
    pub refs: HeaderRefs<'a>,
    pub hostio_fields: HostioFields,
}

impl<'a> GlobalArgs<'a> {
    pub fn new(reader: &mut ArgsReader<'a>) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::from_reader_with_ctx(reader, ())
            .map_err(|_| GoblinError::InvalidPayload)?;
        let header = Header::from_reader_with_ctx(reader, &flags)
            .map_err(|_| GoblinError::InvalidPayload)?;

        // Zero-copy refs borrow the calldata. A `DekuReader` impl cannot recover
        // that slice from a generic reader, so hand it in through the ctx.
        let source: &'a [u8] = reader.as_mut().get_ref();
        let refs = HeaderRefs::from_reader_with_ctx(reader, HeaderRefsCtx { flags, source })
            .map_err(|_| GoblinError::InvalidPayload)?;
        let hostio_fields = HostioFields::try_new(flags.read_msg_value)?;

        Ok(Self {
            flags,
            header,
            refs,
            hostio_fields,
        })
    }

    pub fn process(
        &self,
        reader: &mut ArgsReader<'_>,
        delta: &mut StaticDelta,
    ) -> Result<(), GoblinError> {
        let caller = &self.hostio_fields.msg_sender;

        for_axes!(M, TM0, TM1 => process_market::<(M, Pair<TM0, TM1>)>(
            caller,
            reader,
            &self.refs.token_data_triple,
            &self.header.market_counts,
            delta,
        )?);

        for_axes!(PT, TM0 => PT::settle::<TM0>(
            &delta.global,
            &self.refs.token_data_triple,
            &self.msg_transfers(),
            CallerAddresses {
                caller: &self.hostio_fields.msg_sender,
                custom_recipient: self.refs.custom_recipient
            },
        )?);

        Ok(())
    }

    fn msg_transfers(&self) -> MsgTransfers {
        MsgTransfers::from(ETHTransfers {
            msg_value: self.hostio_fields.msg_value,
            eth_out_due: self.header.eth_out_due_u32.into(),
        })
    }
}
