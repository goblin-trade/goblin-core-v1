use crate::{
    axis::token::token_marker::TokenMarker,
    goblin_error::GoblinError,
    match_axes,
    quantities::{Exp, IntoAbs, Quantity, UnsidedDeltaAtoms},
    settlement::ConstDefault,
    types::{Address, Marker, Tuple},
};

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

// TODO switch order, should decrease be at 0?
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UpdateEnum {
    /// Increase store balance by decreasing resting order
    Increase = 0,

    /// Decrease store balance by increasing resting order
    Decrease = 1,
}

impl From<bool> for UpdateEnum {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Increase,
            true => Self::Decrease,
        }
    }
}

impl TryFrom<u8> for UpdateEnum {
    type Error = GoblinError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UpdateEnum::Increase),
            1 => Ok(UpdateEnum::Decrease),
            _ => Err(GoblinError::InvalidEnumVariant),
        }
    }
}

impl UpdateEnum {
    // TODO move elsewhere
    pub fn transfer<T: TokenMarker>(
        net_deposit: UnsidedDeltaAtoms,
        trader: &Address,
        token_address: &T::TokenAddress,
        decimals: T::StoredDecimals,
    ) -> Result<(), GoblinError> {
        let Some(update_enum) = UpdateEnum::from_delta(net_deposit) else {
            return Ok(());
        };

        let deposit = net_deposit.abs();

        match_axes!(UM = update_enum => {
            T::update::<UM>(deposit, trader, token_address, decimals)?;
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
