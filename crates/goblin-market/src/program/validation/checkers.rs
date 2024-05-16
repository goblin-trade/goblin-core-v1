use stylus_sdk::msg;

use crate::{
    parameters::FEE_COLLECTOR,
    program::error::{GoblinError, GoblinResult, InvalidFeeCollector},
    require,
};

pub fn assert_valid_fee_collector() -> GoblinResult<()> {
    require!(
        msg::sender() == FEE_COLLECTOR,
        GoblinError::InvalidFeeCollector(InvalidFeeCollector {})
    );

    Ok(())
}
