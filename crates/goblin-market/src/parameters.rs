// Market pub constants
// Markets are namespaced by base token, quote token, base lot size, quote lot size,
// base decimals to ignore, quote decimals to ignore, taker fee bps
// decimal fields are unnecessary
// raw_base_lots_per_base lot is not needed here. It is used to derive base lot size

use stylus_sdk::alloy_primitives::{address, Address};

use crate::quantities::{
    BaseAtomsPerBaseLot, BaseLotsPerBaseUnit, QuoteAtomsPerQuoteLot, QuoteLotsPerBaseUnitPerTick,
};

pub const FEE_COLLECTOR: Address = address!("1D9ff13fC7Bea07E6e1C323Ed48521DA532596d8");

pub const BASE_TOKEN: Address = address!("82af49447d8a07e3bd95bd0d56f35241523fbab1");
pub const QUOTE_TOKEN: Address = address!("af88d065e77c8cC2239327C5EDb3A432268e5831");

// Base token (ETH) unit is considered to have 10^8 atoms
// 10^8 as U256 in big endian
pub const BASE_DECIMALS_TO_IGNORE: [u64; 4] = [100000000, 0, 0, 0];

// 0 decimals to ignore- 10^0 = 1
pub const QUOTE_DECIMALS_TO_IGNORE: [u64; 4] = [1, 0, 0, 0];

pub const BASE_LOT_SIZE: BaseAtomsPerBaseLot = BaseAtomsPerBaseLot { inner: 10_000 };
pub const QUOTE_LOT_SIZE: QuoteAtomsPerQuoteLot = QuoteAtomsPerQuoteLot { inner: 1 };

// base lots per base unit = atoms per unit / atoms per lot
// = 10^8 / 10000 = 10^4
pub const BASE_LOTS_PER_BASE_UNIT: BaseLotsPerBaseUnit = BaseLotsPerBaseUnit { inner: 10_000 };

pub const TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT: QuoteLotsPerBaseUnitPerTick =
    QuoteLotsPerBaseUnitPerTick { inner: 10_000 };
// derive tick_size_in_quote_atoms_per_base_unit = TICK_SIZE_IN_QUOTE_LOTS_PER_BASE_UNIT * QUOTE_LOT_SIZE;

// orderbook invariant
// tick size % base lots per base unit (base lot size) = 0, i.e. tick size > base lots per base unit
// = 10^4 % 10^4

// 2 bps fee
pub const TAKER_FEE_BPS: u16 = 2;

#[cfg(test)]
mod test {
    use stylus_sdk::alloy_primitives::U256;

    #[test]
    fn get_decimals_to_ignore() {
        let factor = U256::from(10).pow(U256::from(8));
        println!("factor {:?}", factor);

        let limbs = factor.into_limbs();
        println!("limbs {:?}", limbs);

        let reconstructed_factor = U256::from_limbs(limbs);
        println!("reconstructed_factor {:?}", reconstructed_factor);
    }
}
