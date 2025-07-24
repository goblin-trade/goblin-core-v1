#[repr(C)]
pub enum TokenPair {
    EthAsBase(EthAsBase),
    EthAsQuote(EthAsQuote),
    TokenToken(TokenToken),
}

#[repr(C, packed)]
pub struct EthAsBase {
    pub quote_token_index: u8,
}

#[repr(C, packed)]
pub struct EthAsQuote {
    pub base_token: u8,
}

#[repr(C, packed)]
pub struct TokenToken {
    pub base_token_index: u8,
    pub quote_token_index: u8,
}
