use crate::{
    axis::{
        leg::Pair,
        market::{market_counts::MarketCounts, Dynamic, Hardcoded},
        token::{HardcodedERC20, ETH},
    },
    for_axes,
    goblin_error::GoblinError,
    input_processor::{
        global_args::{global_header::GlobalHeader, hostio_fields::HostioFields},
        ArgsReader, ETHTransfers, HeaderFlags, MsgTransfers,
    },
    market::process_market_outer,
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
        // let counts = self.global_header.market_counts;

        // problem- trait HardcodedMarketList is not implemented for illegal combinations like
        // (Hardcoded, Pair<ETH, ETH>)
        process_market_outer::<(Hardcoded, Pair<ETH, ETH>)>(
            &self.hostio_fields.msg_sender,
            reader,
            &self.global_header.token_data_triple,
            delta,
            &self.global_header.market_counts,
        )?;

        // for_axes!(MM, B, Q => process_market_outer::<(MM, Pair<B, Q>)>(
        //     &self.hostio_fields.msg_sender,
        //     reader,
        //     &self.global_header.token_data_triple,
        //     delta,
        //     &self.global_header.market_counts,
        // )?);

        // TODO remove duplication along with internal count reads
        // problem- if we use for_axes! for 18 combinations, we will need
        // 18 bytes.
        //
        // Currently we only use 2 + 6 = 8 bytes
        // Hardcoded::process(
        //     &self.global_header.market_counts,
        //     &self.hostio_fields.msg_sender,
        //     reader,
        //     &self.global_header.token_data_triple,
        //     delta,
        // )?;

        // Dynamic::process(
        //     &self.global_header.market_counts,
        //     &self.hostio_fields.msg_sender,
        //     reader,
        //     &self.global_header.token_data_triple,
        //     delta,
        // )?;

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
