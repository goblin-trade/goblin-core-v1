use crate::define_custom_type;

// Inner bitmap
define_custom_type!(InnerPos<u8>);
define_custom_type!(Row<u8>);
define_custom_type!(Column<u8>);

impl From<InnerPos> for Row {
    fn from(value: InnerPos) -> Self {
        Row(value.0 / 8)
    }
}

impl From<InnerPos> for Column {
    fn from(value: InnerPos) -> Self {
        Column(value.0 % 8)
    }
}
