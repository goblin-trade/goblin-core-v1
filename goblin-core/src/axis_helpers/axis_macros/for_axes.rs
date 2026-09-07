#[macro_export]
macro_rules! for_axes {
    ( $($axis:ident),+ $(,)? => $body:block ) => {
        $crate::for_axes!(@expand [$($axis),+] [] $body)
    };

    ( $($axis:ident),+ $(,)? => $body:expr ) => {
        $crate::for_axes!(@expand [$($axis),+] [] { $body })
    };

    (@expand [TM0 $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range TM0, $crate::axis::token::Token, 3, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [TM1 $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range TM1, $crate::axis::token::Token, 3, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [In $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range In, $crate::axis::leg::Leg, 2, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [M $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range M, $crate::axis::market::Market, 2, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [OM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range OM, $crate::axis::occupancy::Occupancy, 2, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [UM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range UM, $crate::axis::update::Update, 2, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [PT $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range PT, $crate::axis::party::Party, 2, [$($rest),*], [$($alias)*], $body);
    };
    (@expand [CM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range CM, $crate::axis::caller::Caller, 2, [$($rest),*], [$($alias)*], $body);
    };

    (@range $axis:ident, $seed:path, 2, [$($rest:ident),*], [$($alias:item)*], $body:block) => {
        $crate::for_axes!(@expand [$($rest),*] [$($alias)* type $axis = $crate::types::Marker<$seed, 0>;] $body);
        $crate::for_axes!(@expand [$($rest),*] [$($alias)* type $axis = $crate::types::Marker<$seed, 1>;] $body);
    };
    (@range $axis:ident, $seed:path, 3, [$($rest:ident),*], [$($alias:item)*], $body:block) => {
        $crate::for_axes!(@expand [$($rest),*] [$($alias)* type $axis = $crate::types::Marker<$seed, 0>;] $body);
        $crate::for_axes!(@expand [$($rest),*] [$($alias)* type $axis = $crate::types::Marker<$seed, 1>;] $body);
        $crate::for_axes!(@expand [$($rest),*] [$($alias)* type $axis = $crate::types::Marker<$seed, 2>;] $body);
    };

    (@expand [] [$($alias:item)*] $body:block) => {
        {
            $($alias)*
            $body
        }
    };
}
