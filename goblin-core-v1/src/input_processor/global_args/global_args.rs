use crate::{
    axis::market::{market_counts::MarketCounts, Dynamic, Hardcoded},
    goblin_error::GoblinError,
    input_processor::{
        global_args::{global_header::GlobalHeader, hostio_fields::HostioFields},
        DecodeCtx, ETHTransfers, FixedDecode, HeaderFlags, MsgTransfers, VariableDecode,
    },
    settlement::Delta,
    types::Address,
};

pub struct GlobalArgs<'a> {
    pub flags: HeaderFlags,
    pub global_header: GlobalHeader<'a>,
    pub hostio_fields: HostioFields,
}

impl<'a> GlobalArgs<'a> {
    pub fn try_new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        let flags = HeaderFlags::try_fixed_decode(ctx)?;
        let global_header = GlobalHeader::try_variable_decode(ctx, &flags)?;
        let hostio_fields = HostioFields::try_new(flags.read_msg_value)?;

        Ok(Self {
            flags,
            global_header,
            hostio_fields,
        })
    }

    pub fn process(&'a self, ctx: &DecodeCtx, delta: &mut Delta) -> Result<(), GoblinError> {
        // TODO remove duplication along with internal count reads
        Hardcoded::process(
            &self.global_header.market_counts,
            &self.hostio_fields.msg_sender,
            ctx,
            &self.global_header.token_data_triple,
            delta,
        )?;

        Dynamic::process(
            &self.global_header.market_counts,
            &self.hostio_fields.msg_sender,
            ctx,
            &self.global_header.token_data_triple,
            delta,
        )?;

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
