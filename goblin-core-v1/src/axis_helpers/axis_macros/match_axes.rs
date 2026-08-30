#[macro_export]
macro_rules! match_axes {
    ( $($axis:ident = $val:expr),+ $(,)? => $body:block ) => {
        $crate::match_axes!(@expand [$($axis = $val),+] $body)
    };

    (@expand [] $body:block) => { $body };

    // --- Axis -> (arity, seed type). No enum path, no variant names. ---
    (@expand [In = $val:expr $(, $axis:ident = $rest_val:expr)*] $body:block) => {
        $crate::match_axes!(@arity 2, $val, In, $crate::axis::leg::Leg, [$($axis = $rest_val),*], $body)
    };
    (@expand [UM = $val:expr $(, $axis:ident = $rest_val:expr)*] $body:block) => {
        $crate::match_axes!(@arity 2, $val, UM, $crate::axis::update::Update, [$($axis = $rest_val),*], $body)
    };
    (@expand [OM = $val:expr $(, $axis:ident = $rest_val:expr)*] $body:block) => {
        $crate::match_axes!(@arity 2, $val, OM, $crate::axis::occupancy::Occupancy, [$($axis = $rest_val),*], $body)
    };
    (@expand [M = $val:expr $(, $axis:ident = $rest_val:expr)*] $body:block) => {
        $crate::match_axes!(@arity 2, $val, M, $crate::axis::market::Market, [$($axis = $rest_val),*], $body)
    };
    (@expand [TM = $val:expr $(, $axis:ident = $rest_val:expr)*] $body:block) => {
        $crate::match_axes!(@arity 3, $val, TM, $crate::axis::token::Token, [$($axis = $rest_val),*], $body)
    };

    // --- Shared: branch on the enum's numeric discriminant, not its variant path ---
    (@arity 2, $val:expr, $axis:ident, $seed:path, $rest:tt, $body:block) => {
        match $val as usize {
            0 => { type $axis = $crate::types::Marker<$seed, 0>; $crate::match_axes!(@expand $rest $body) }
            1 => { type $axis = $crate::types::Marker<$seed, 1>; $crate::match_axes!(@expand $rest $body) }
            _ => unreachable!("axis `{}` discriminant out of range", stringify!($axis)),
        }
    };

    (@arity 3, $val:expr, $axis:ident, $seed:path, $rest:tt, $body:block) => {
        match $val as usize {
            0 => { type $axis = $crate::types::Marker<$seed, 0>; $crate::match_axes!(@expand $rest $body) }
            1 => { type $axis = $crate::types::Marker<$seed, 1>; $crate::match_axes!(@expand $rest $body) }
            2 => { type $axis = $crate::types::Marker<$seed, 2>; $crate::match_axes!(@expand $rest $body) }
            _ => unreachable!("axis `{}` discriminant out of range", stringify!($axis)),
        }
    };
}
