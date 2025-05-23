use crate::{goblin_error::GoblinError, require};

pub fn read_custom_tokens<'a>(
    input: &'a [u8; 512],
    len: usize,
    offset: &mut usize,
) -> &'a [[u8; 20]] {
    let custom_token_count = input[*offset];
    *offset += 1;

    // Reduce limit to 16, that way size occupied is 16 * 20 = 320 bytes
    require!(
        custom_token_count < 16,
        GoblinError::CustomTokenLimitExceeded
    );
    let token_list_size = custom_token_count as usize * 20;
    require!(len > *offset + token_list_size, GoblinError::InvalidPayload);

    let payload = &input[*offset..*offset + token_list_size];
    *offset += token_list_size;

    let custom_tokens = unsafe {
        core::slice::from_raw_parts(
            payload.as_ptr() as *const [u8; 20],
            custom_token_count as usize,
        )
    };

    custom_tokens
}
