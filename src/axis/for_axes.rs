#[macro_export]
macro_rules! for_axes {
    // entry: one or more canonical axis names, no explicit axis label needed
    (|$first:ident $(, $rest:ident)*| $($body:tt)+) => {
        $crate::for_axes!(@peel [$first $(, $rest)*] [] [] [] [] [] { $($body)+ });
    };

    // base case: all names resolved -> substitute into body
    (@peel [] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [] $($body)+);
    };

    // TM -> TokenMarker
    (@peel [TM $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::for_axes!(@peel [$($rest),*] [$crate::axis::token::ETH]            [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$crate::axis::token::HardcodedERC20] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$crate::axis::token::CustomERC20]    [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] { $($body)+ });
    };

    // LM -> LegMatcher
    (@peel [LM $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$crate::axis::leg::Base]  [$($mm)*] [$($om)*] [$($um)*] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$crate::axis::leg::Quote] [$($mm)*] [$($om)*] [$($um)*] { $($body)+ });
    };

    // MM -> MarketMarker
    (@peel [MM $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$crate::axis::market::Hardcoded] [$($om)*] [$($um)*] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$crate::axis::market::Dynamic]   [$($om)*] [$($um)*] { $($body)+ });
    };

    // OM -> OccupancyMarker
    (@peel [OM $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$($mm)*] [$crate::axis::occupancy::Vacant]   [$($um)*] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$($mm)*] [$crate::axis::occupancy::Occupied] [$($um)*] { $($body)+ });
    };

    // UM -> UpdateMarker
    (@peel [UM $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$crate::axis::update::Increase] { $($body)+ });
        $crate::for_axes!(@peel [$($rest),*] [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$crate::axis::update::Decrease] { $($body)+ });
    };
}

// Internal: single-pass substitution of canonical names in body tokens.
#[doc(hidden)]
#[macro_export]
macro_rules! __for_axes_apply {
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*]) => {
        $($acc)* ;
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] TM $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $($tm)*] $($rest)*)
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] LM $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $($lm)*] $($rest)*)
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] MM $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $($mm)*] $($rest)*)
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] OM $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $($om)*] $($rest)*)
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] UM $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $($um)*] $($rest)*)
    };
    ([$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] [$($acc:tt)*] $other:tt $($rest:tt)*) => {
        $crate::__for_axes_apply!([$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*] [$($acc)* $other] $($rest)*)
    };
}
