use stylus_sdk::msg;

use crate::{
    error::{GoblinError, GoblinResult, InvalidFeeCollector},
    parameters::FEE_COLLECTOR,
    require,
};

pub fn assert_valid_fee_collector() -> GoblinResult<()> {
    require!(
        msg::sender() == FEE_COLLECTOR,
        GoblinError::InvalidFeeCollector(InvalidFeeCollector {})
    );

    Ok(())
}
