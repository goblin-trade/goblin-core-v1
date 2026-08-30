use crate::{
    axis::token::token_marker::TokenMarker,
    define_axis,
    goblin_error::GoblinError,
    match_axes,
    quantities::{Exp, IntoAbs, Quantity, UnsidedDeltaAtoms},
    settlement::ConstDefault,
    types::{Address, Tuple},
};

define_axis! {
    pub struct Update;
    enum UpdateEnum {
        Increase = 0,
        Decrease = 1,
    }
}

impl UpdateEnum {
    // TODO move elsewhere
    pub fn transfer<TM: TokenMarker>(
        net_deposit: UnsidedDeltaAtoms,
        trader: &Address,
        token_address: &TM::TokenAddress,
        decimals: TM::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let Some(update_enum) = UpdateEnum::from_delta(net_deposit) else {
            return Ok(());
        };

        let deposit = net_deposit.abs();

        match_axes!(UM = update_enum => {
            TM::update::<UM>(deposit, trader, token_address, decimals)?;
        });

        Ok(())
    }

    fn from_delta<E: Exp>(value: Quantity<E, i64>) -> Option<Self> {
        if value > Quantity::DEFAULT {
            Some(Self::Increase)
        } else if value < Quantity::DEFAULT {
            Some(Self::Decrease)
        } else {
            None
        }
    }
}

pub type UpdatePair<T0, T1> = Tuple<T0, T1, Update>;
pub type SameUpdatePair<T> = UpdatePair<T, T>;
