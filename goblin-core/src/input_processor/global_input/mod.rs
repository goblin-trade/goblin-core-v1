pub mod caller_addresses;
pub mod global_args;
pub mod header;
pub mod header_flags;
pub mod header_refs;

pub use caller_addresses::*;
pub use global_args::*;
pub use header::*;
pub use header_flags::*;
pub use header_refs::*;

mod hostio_fields;

use hostio_fields::HostioFields;

use crate::{
    axis::{
        leg::Pair,
        party::PartySettle,
        token::{ETHTransfers, MsgTransfers},
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::process_market,
    settlement::StaticDelta,
};

/// The complete set of global inputs to a call: the calldata [`GlobalArgs`]
/// together with the fields read from hostio.
///
/// [`GlobalArgs`] is a pure calldata payload that implements Deku, so anything
/// that is not part of that payload (hostio values) lives here instead. The
/// settlement logic that consumes both also lives here.
pub struct GlobalInput<'a> {
    pub args: GlobalArgs<'a>,
    pub hostio_fields: HostioFields,
}

impl<'a> GlobalInput<'a> {
    pub fn new(reader: &mut ArgsReader<'a>) -> Result<Self, GoblinError> {
        let args = GlobalArgs::new(reader)?;
        let hostio_fields = HostioFields::try_new(args.flags.read_msg_value)?;

        Ok(Self {
            args,
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
            &self.args.refs.token_data_triple,
            &self.args.header.market_counts,
            delta,
        )?);

        for_axes!(PT, TM0 => PT::settle::<TM0>(
            &delta.global,
            &self.args.refs.token_data_triple,
            &self.msg_transfers(),
            CallerAddresses {
                caller: &self.hostio_fields.msg_sender,
                custom_recipient: self.args.refs.custom_recipient
            },
        )?);

        Ok(())
    }

    fn msg_transfers(&self) -> MsgTransfers {
        MsgTransfers::from(ETHTransfers {
            msg_value: self.hostio_fields.msg_value,
            eth_out_due: self.args.header.eth_out_due_u32.into(),
        })
    }
}
