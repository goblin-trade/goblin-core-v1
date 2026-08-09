use crate::{
    axis::token::token_marker::TokenMarker,
    goblin_error::GoblinError,
    quantities::{Exp, IntoAbs, Quantity, UnsidedDeltaAtoms},
    settlement::ConstDefault,
    types::{Address, Marker, Tuple},
};

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

#[derive(Clone, Copy)]
pub enum UpdateEnum {
    /// Increase store balance by decreasing resting order
    Increase,

    /// Decrease store balance by increasing resting order
    Decrease,
}

impl UpdateEnum {
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
        match update_enum {
            UpdateEnum::Increase => T::update::<Increase>(deposit, trader, token_address, decimals),
            UpdateEnum::Decrease => T::update::<Decrease>(deposit, trader, token_address, decimals),
        }
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
