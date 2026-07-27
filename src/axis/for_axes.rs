use crate::axis::{
    leg::{Base, Quote},
    market::{Dynamic, Hardcoded},
};

#[macro_export]
macro_rules! for_axes {
    // entry point: closure-like syntax
    (|$var:ident : $axis:ident $(, $rvar:ident : $raxis:ident)*| $body:expr) => {
        for_axes!(@peel [$var : $axis $(, $rvar : $raxis)*] [] { $body });
    };

    // base case: all axes bound, emit one specialized block
    (@peel [] [$($alias:item)*] { $body:expr }) => {
        { $($alias)* $body };
    };

    // one arm per axis, each peels off the first (var, axis) pair
    (@peel [$var:ident : Token $(, $rvar:ident : $raxis:ident)*] [$($alias:item)*] { $body:expr }) => {
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = ETH;]            { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = HardcodedERC20;] { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = CustomERC20;]    { $body });
    };

    (@peel [$var:ident : Leg $(, $rvar:ident : $raxis:ident)*] [$($alias:item)*] { $body:expr }) => {
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Base;]  { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Quote;] { $body });
    };

    (@peel [$var:ident : Market $(, $rvar:ident : $raxis:ident)*] [$($alias:item)*] { $body:expr }) => {
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Hardcoded;] { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Dynamic;]   { $body });
    };

    (@peel [$var:ident : Occupancy $(, $rvar:ident : $raxis:ident)*] [$($alias:item)*] { $body:expr }) => {
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Vacant;]   { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Occupied;] { $body });
    };

    (@peel [$var:ident : Update $(, $rvar:ident : $raxis:ident)*] [$($alias:item)*] { $body:expr }) => {
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Increase;] { $body });
        for_axes!(@peel [$($rvar : $raxis),*] [$($alias)* type $var = Decrease;] { $body });
    };
}
