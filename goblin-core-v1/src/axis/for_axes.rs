#[macro_export]
macro_rules! for_axes {
    ( | $($axis:ident),+ $(,)? | $($body:tt)+ ) => {
        $crate::for_axes!(@expand [$($axis),+] [] { $($body)+ });
    };

    // --- Branch generation ---

    (@expand [TM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type TM = $crate::axis::token::ETH;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type TM = $crate::axis::token::HardcodedERC20;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type TM = $crate::axis::token::CustomERC20;] $body);
    };

    (@expand [In $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type In = $crate::axis::leg::Base;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type In = $crate::axis::leg::Quote;] $body);
    };

    (@expand [MM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type MM = $crate::axis::market::Hardcoded;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type MM = $crate::axis::market::Dynamic;] $body);
    };

    (@expand [OM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type OM = $crate::axis::occupancy::Vacant;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type OM = $crate::axis::occupancy::Occupied;] $body);
    };

    (@expand [UM $(, $rest:ident)*] [$($alias:item)*] $body:block) => {
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type UM = $crate::axis::update::Increase;] $body);
        $crate::for_axes!(@expand [$($rest)*] [$($alias)* type UM = $crate::axis::update::Decrease;] $body);
    };

    // --- Base case: emit block with scoped type aliases ---
    (@expand [] [$($alias:item)*] $body:block) => {
        {
            $($alias)*
            $body
        }
    };
}
