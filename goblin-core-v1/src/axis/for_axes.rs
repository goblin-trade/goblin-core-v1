#[macro_export]
macro_rules! for_axes {
    ( | $($axis:ident),+ $(,)? | $($body:tt)+ ) => {
        $crate::for_axes!(@expand [$($axis),+] [] { $($body)+ });
    };

    // --- Axis -> (seed type, arity) ---
    (@expand [TM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range TM, $crate::axis::token::Token, 3, [$($rest)*], [$($alias)*], $body);
    };
    (@expand [In $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range In, $crate::axis::leg::Leg, 2, [$($rest)*], [$($alias)*], $body);
    };
    (@expand [MM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range MM, $crate::axis::market::Market, 2, [$($rest)*], [$($alias)*], $body);
    };
    (@expand [OM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range OM, $crate::axis::occupancy::Occupancy, 2, [$($rest)*], [$($alias)*], $body);
    };
    (@expand [UM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@range UM, $crate::axis::update::Update, 2, [$($rest)*], [$($alias)*], $body);
    };

    // --- Shared by every axis of a given arity ---
    (@range $axis:ident, $seed:path, 2, [$($rest:ident)*], [$($alias:item)*], $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type $axis = $crate::types::Marker<$seed, 0>;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type $axis = $crate::types::Marker<$seed, 1>;] $body);
    };
    (@range $axis:ident, $seed:path, 3, [$($rest:ident)*], [$($alias:item)*], $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type $axis = $crate::types::Marker<$seed, 0>;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type $axis = $crate::types::Marker<$seed, 1>;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type $axis = $crate::types::Marker<$seed, 2>;] $body);
    };

    // --- Base case ---
    (@expand [] [$($alias:item)*] $body:block) => {
        { $($alias)* $body }
    };
}
