use crate::{
    token::{CustomToken, HardcodedToken, ERC20, ETH},
    types::{Base, Quote, Tuple, TupleReader},
};

/// Macro to implement TupleReader for a pair of types
///
/// Usage: `impl_tuple_reader!(Type0, Type1);`
///
/// This will create implementations where:
/// - Type0 reads the first element (tuple.0)
/// - Type1 reads the second element (tuple.1)
macro_rules! impl_tuple_reader {
    ($t0:ty, $t1:ty) => {
        // Implementation for the first type (T0)
        impl<T0, T1> TupleReader<T0, T1, ($t0, $t1)> for $t0
        where
            T0: Clone + Copy,
            T1: Clone + Copy,
        {
            type Result = T0;

            fn get(tuple: &Tuple<T0, T1, ($t0, $t1)>) -> Self::Result {
                tuple.0
            }

            fn get_leg(tuple: &Tuple<T0, T1, ($t0, $t1)>) -> &Self::Result {
                &tuple.0
            }

            fn get_leg_mut(tuple: &mut Tuple<T0, T1, ($t0, $t1)>) -> &mut Self::Result {
                &mut tuple.0
            }
        }

        // Implementation for the second type (T1)
        impl<T0, T1> TupleReader<T0, T1, ($t0, $t1)> for $t1
        where
            T0: Clone + Copy,
            T1: Clone + Copy,
        {
            type Result = T1;

            fn get(tuple: &Tuple<T0, T1, ($t0, $t1)>) -> Self::Result {
                tuple.1
            }

            fn get_leg(tuple: &Tuple<T0, T1, ($t0, $t1)>) -> &Self::Result {
                &tuple.1
            }

            fn get_leg_mut(tuple: &mut Tuple<T0, T1, ($t0, $t1)>) -> &mut Self::Result {
                &mut tuple.1
            }
        }
    };
}

// Apply the macro to create implementations for all desired pairs
impl_tuple_reader!(ETH, ERC20);
impl_tuple_reader!(Base, Quote);
impl_tuple_reader!(HardcodedToken, CustomToken);
