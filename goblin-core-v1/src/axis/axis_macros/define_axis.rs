#[macro_export]
macro_rules! define_axis {
    // --- binary axis ---
    (
        $(#[$meta:meta])*
        $vis:vis struct $seed:ident;
        enum $enum_name:ident {
            $v0:ident = 0,
            $v1:ident = 1,
        }
    ) => {
        $(#[$meta])*
        #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $seed;

        #[repr(u8)]
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $enum_name {
            $v0 = 0,
            $v1 = 1,
        }

        impl ::core::convert::From<bool> for $enum_name {
            fn from(value: bool) -> Self {
                if value { Self::$v1 } else { Self::$v0 }
            }
        }

        impl ::core::convert::TryFrom<u8> for $enum_name {
            type Error = $crate::goblin_error::GoblinError;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::$v0),
                    1 => Ok(Self::$v1),
                    _ => Err($crate::goblin_error::GoblinError::InvalidEnumVariant),
                }
            }
        }

        pub type $v0 = $crate::types::Marker<$seed, { $enum_name::$v0 as usize }>;
        pub type $v1 = $crate::types::Marker<$seed, { $enum_name::$v1 as usize }>;

        impl $crate::axis::AxisMarker for $v0 {
            type Axis = $seed;
            type Value = $enum_name;
            const VALUE: Self::Value = $enum_name::$v0;
        }
        impl $crate::axis::AxisMarker for $v1 {
            type Axis = $seed;
            type Value = $enum_name;
            const VALUE: Self::Value = $enum_name::$v1;
        }
    };

    // --- triple axis ---
    (
        $(#[$meta:meta])*
        $vis:vis struct $seed:ident;
        enum $enum_name:ident {
            $v0:ident = 0,
            $v1:ident = 1,
            $v2:ident = 2,
        }
    ) => {
        $(#[$meta])*
        #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $seed;

        #[repr(u8)]
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $enum_name {
            $v0 = 0,
            $v1 = 1,
            $v2 = 2,
        }

        impl ::core::convert::TryFrom<u8> for $enum_name {
            type Error = $crate::goblin_error::GoblinError;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::$v0),
                    1 => Ok(Self::$v1),
                    2 => Ok(Self::$v2),
                    _ => Err($crate::goblin_error::GoblinError::InvalidEnumVariant),
                }
            }
        }

        pub type $v0 = $crate::types::Marker<$seed, { $enum_name::$v0 as usize }>;
        pub type $v1 = $crate::types::Marker<$seed, { $enum_name::$v1 as usize }>;
        pub type $v2 = $crate::types::Marker<$seed, { $enum_name::$v2 as usize }>;

        impl $crate::axis::AxisMarker for $v0 {
            type Axis = $seed;
            type Value = $enum_name;
            const VALUE: Self::Value = $enum_name::$v0;
        }
        impl $crate::axis::AxisMarker for $v1 {
            type Axis = $seed;
            type Value = $enum_name;
            const VALUE: Self::Value = $enum_name::$v1;
        }
        impl $crate::axis::AxisMarker for $v2 {
            type Axis = $seed;
            type Value = $enum_name;
            const VALUE: Self::Value = $enum_name::$v2;
        }
    };
}
