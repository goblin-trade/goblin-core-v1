/// Macro to implement TripleReader for a triple of types
///
/// Usage: `impl_triple_reader!(Type0, Type1, Type2);`
///
/// This will create implementations where:
/// - Type0 reads the first element (triple.0)
/// - Type1 reads the second element (triple.1)
/// - Type2 reads the third element (triple.2)
#[macro_export]
macro_rules! impl_triple_reader {
    ($t0:ty, $t1:ty, $t2:ty) => {
        // Implementation for the first type (T0)
        impl<T0, T1, T2> TripleReader<T0, T1, T2, ($t0, $t1, $t2)> for $t0
        where
            T0: Clone + Copy,
            T1: Clone + Copy,
            T2: Clone + Copy,
        {
            type Result = T0;

            fn get(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> Self::Result {
                triple.0
            }

            fn get_leg(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &Self::Result {
                &triple.0
            }

            fn get_leg_mut(triple: &mut Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &mut Self::Result {
                &mut triple.0
            }
        }

        // Implementation for the second type (T1)
        impl<T0, T1, T2> TripleReader<T0, T1, T2, ($t0, $t1, $t2)> for $t1
        where
            T0: Clone + Copy,
            T1: Clone + Copy,
            T2: Clone + Copy,
        {
            type Result = T1;

            fn get(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> Self::Result {
                triple.1
            }

            fn get_leg(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &Self::Result {
                &triple.1
            }

            fn get_leg_mut(triple: &mut Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &mut Self::Result {
                &mut triple.1
            }
        }

        // Implementation for the third type (T2)
        impl<T0, T1, T2> TripleReader<T0, T1, T2, ($t0, $t1, $t2)> for $t2
        where
            T0: Clone + Copy,
            T1: Clone + Copy,
            T2: Clone + Copy,
        {
            type Result = T2;

            fn get(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> Self::Result {
                triple.2
            }

            fn get_leg(triple: &Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &Self::Result {
                &triple.2
            }

            fn get_leg_mut(triple: &mut Triple<T0, T1, T2, ($t0, $t1, $t2)>) -> &mut Self::Result {
                &mut triple.2
            }
        }
    };
}
