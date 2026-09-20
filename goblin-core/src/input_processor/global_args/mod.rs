pub mod caller_addresses;
pub mod header;
pub mod header_flags;
pub mod header_refs;

pub use caller_addresses::*;
pub use header::*;
pub use header_flags::*;
pub use header_refs::*;

mod hostio_fields;
mod impl_compound_decode;

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
    pub fn process(
        &'a self,
        reader: &ArgsReader,
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
