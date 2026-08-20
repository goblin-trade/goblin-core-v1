use crate::{
    axis::{
        leg::Pair,
        market::{
            market_counts::{MarketCounts, MarketCountsTuple},
            Hardcoded,
        },
        token::{token_reader::TokenDataTriple, HardcodedERC20, ETH},
    },
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::process_market,
    settlement::StaticDelta,
    types::{Address, StoreReader},
};

impl MarketCounts for Hardcoded {
    fn process<'a>(
        market_counts: &MarketCountsTuple,
        msg_sender: &Address,
        reader: &ArgsReader,
        token_data_triple: &TokenDataTriple<'a>,
        static_delta: &mut StaticDelta,
    ) -> Result<(), GoblinError> {
        let counts = Self::get_leg(market_counts);

        for _ in 0..counts.inner[0] {
            process_market::<(Hardcoded, Pair<ETH, HardcodedERC20>)>(
                msg_sender,
                reader,
                token_data_triple,
                static_delta,
            )?;
        }

        for _ in 0..counts.inner[1] {
            process_market::<(Hardcoded, Pair<HardcodedERC20, ETH>)>(
                msg_sender,
                reader,
                token_data_triple,
                static_delta,
            )?;
        }

        for _ in 0..counts.inner[2] {
            process_market::<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)>(
                msg_sender,
                reader,
                token_data_triple,
                static_delta,
            )?;
        }

        Ok(())
    }
}
