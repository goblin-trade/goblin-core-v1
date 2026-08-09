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

    // In -> LegMatcher
    (@peel [In $(, $rest:ident)*] [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*] { $($body:tt)+ }) => {
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

// Internal: recursive stack-based substitution of canonical names in body tokens.
#[doc(hidden)]
#[macro_export]
macro_rules! __for_axes_apply {
    // Entry point: initialize stack []
    (
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        []
        $($body:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [] // Stack of parent contexts
            [] // Accumulator for current level
            $($body)*
        );
    };

    // --- RECURSIVE GROUP DESCENT ---

    // Parentheses (...)
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        ($($inner:tt)*) $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [(@paren [$($acc)*] [$($rest)*]) $($stack)*]
            []
            $($inner)*
        );
    };

    // Braces {...}
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        {$($inner:tt)*} $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om:tt)*] [$($um)*]
            [(@brace [$($acc)*] [$($rest)*]) $($stack)*]
            []
            $($inner)*
        );
    };

    // Brackets [...]
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        [$($inner:tt)*] $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [(@bracket [$($acc)*] [$($rest)*]) $($stack)*]
            []
            $($inner)*
        );
    };

    // --- IDENTIFIER REPLACEMENTS ---

    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        TM $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $($tm)*]
            $($rest)*
        );
    };

    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        In $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $($lm)*]
            $($rest)*
        );
    };

    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        MM $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $($mm)*]
            $($rest)*
        );
    };

    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        OM $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $($om)*]
            $($rest)*
        );
    };

    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        UM $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $($um)*]
            $($rest)*
        );
    };

    // Default single token pass-through
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [$($stack:tt)*]
        [$($acc:tt)*]
        $other:tt $($rest:tt)*
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack)*]
            [$($acc)* $other]
            $($rest)*
        );
    };

    // --- POPPING STACK FRAMES ---

    // Pop Paren frame
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [(@paren [$($p_acc:tt)*] [$($p_rest:tt)*]) $($stack_tail:tt)*]
        [$($inner_acc:tt)*]
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack_tail)*]
            [$($p_acc)* ( $($inner_acc)* )]
            $($p_rest)*
        );
    };

    // Pop Brace frame
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [(@brace [$($p_acc:tt)*] [$($p_rest:tt)*]) $($stack_tail:tt)*]
        [$($inner_acc:tt)*]
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack_tail)*]
            [$($p_acc)* { $($inner_acc)* }]
            $($p_rest)*
        );
    };

    // Pop Bracket frame
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        [(@bracket [$($p_acc:tt)*] [$($p_rest:tt)*]) $($stack_tail:tt)*]
        [$($inner_acc:tt)*]
    ) => {
        $crate::__for_axes_apply!(
            @munch
            [$($tm)*] [$($lm)*] [$($mm)*] [$($om)*] [$($um)*]
            [$($stack_tail)*]
            [$($p_acc)* [ $($inner_acc)* ]]
            $($p_rest)*
        );
    };

    // Base case: Stack is empty, output full statement
    (
        @munch
        [$($tm:tt)*] [$($lm:tt)*] [$($mm:tt)*] [$($om:tt)*] [$($um:tt)*]
        []
        [$($acc:tt)*]
    ) => {
        $($acc)* ;
    };
}
