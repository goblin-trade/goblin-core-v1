use crate::{
    axis::leg::Pair,
    for_axes,
    goblin_error::GoblinError,
    input_processor::{
        global_args::{global_header::GlobalHeader, hostio_fields::HostioFields},
        ArgsReader, ETHTransfers, HeaderFlags, MsgTransfers,
    },
    market::process_market,
    settlement::StaticDelta,
    types::Address,
};

pub struct GlobalArgs<'a> {
    pub flags: HeaderFlags,
    pub global_header: GlobalHeader<'a>,
    pub hostio_fields: HostioFields,
}

impl<'a> GlobalArgs<'a> {
    pub fn process(
        &'a self,
        reader: &ArgsReader,
        delta: &mut StaticDelta,
    ) -> Result<(), GoblinError> {
        for_axes!(MM, TM0, TM1 => process_market::<(MM, Pair<TM0, TM1>)>(
            &self.hostio_fields.msg_sender,
            reader,
            &self.global_header.token_data_triple,
            &self.global_header.market_counts,
            delta,
        )?);

        delta.global.settle(
            self.recipient(),
            &self.global_header.token_data_triple,
            &self.msg_transfers(),
        )
    }

    fn recipient(&'a self) -> &'a Address {
        self.global_header
            .custom_recipient
            .unwrap_or(&self.hostio_fields.msg_sender)
    }

    fn msg_transfers(&self) -> MsgTransfers {
        MsgTransfers::from(ETHTransfers {
            msg_value: self.hostio_fields.msg_value,
            eth_out_due: self.global_header.eth_out_due,
        })
    }
}
