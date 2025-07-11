pub enum Token {
    ExampleToken1,
    ExampleToken2,
}

impl Token {
    pub const fn address(&self) -> [u8; 20] {
        match self {
            // Example tokens for testnet
            Token::ExampleToken1 => [0x11; 20],
            Token::ExampleToken2 => [0x12; 20],
        }
    }

    pub const fn decimals(&self) -> u8 {
        match self {
            Token::ExampleToken1 => 18,
            Token::ExampleToken2 => 18,
        }
    }
}

pub const HARDCODED_TOKENS: [[u8; 20]; 2] = [
    Token::ExampleToken1.address(),
    Token::ExampleToken2.address(),
];
