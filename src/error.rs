use stylus_sdk::prelude::*;
use alloy_sol_types::sol;

sol! {
    error InvalidInstructionData();
}

#[derive(SolidityError)]
pub enum FairyError {
    InvalidInstructionData(InvalidInstructionData)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_error_bytes() {
        let bytes = Vec::<u8>::from(FairyError::InvalidInstructionData(InvalidInstructionData {}));
        println!("bytes {:?}", bytes);
    }
}