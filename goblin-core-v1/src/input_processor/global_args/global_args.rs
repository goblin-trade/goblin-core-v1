use crate::{
    axis::{leg::Pair, party::PartySettle},
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
        let caller = &self.hostio_fields.msg_sender;

        for_axes!(M, TM0, TM1 => process_market::<(M, Pair<TM0, TM1>)>(
            caller,
            reader,
            &self.global_header.token_data_triple,
            &self.global_header.market_counts,
            delta,
        )?);

        for_axes!(PT, TM0 => PT::settle::<TM0>(
            &delta.global,
            &self.global_header.token_data_triple,
            &self.msg_transfers(),
            caller,
            self.recipient(),
        )?);

        Ok(())
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
