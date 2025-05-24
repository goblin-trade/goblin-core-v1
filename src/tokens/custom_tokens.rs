use crate::{goblin_error::GoblinError, require};

pub fn read_custom_tokens<'a>(
    input: &'a [u8; 512],
    len: usize,
) -> Result<&'a [[u8; 20]], GoblinError> {
    let custom_tokens = if len > 21 {
        let custom_token_count = input[21];

        // Reduce limit to 16, that way size occupied is 16 * 20 = 320 bytes
        require!(
            custom_token_count < 16,
            GoblinError::CustomTokenLimitExceeded
        );

        let token_list_size = custom_token_count as usize * 20;
        require!(len >= 22 + token_list_size, GoblinError::InvalidPayload);

        unsafe {
            core::slice::from_raw_parts(
                input[22..(22 + token_list_size)].as_ptr() as *const [u8; 20],
                custom_token_count as usize,
            )
        }
    } else {
        &[]
    };

    Ok(custom_tokens)
}
