use crate::{
    goblin_error::GoblinError,
    require,
    state::{ERC20Store, ERC20StoreKey, EthStore, EthStoreKey, SlotState},
    tokens::{ERC20Token, Token, TokenIndex},
    types::{Address, SideMarker},
};

pub struct ERC20TokenPair {
    pub base_token: ERC20Token,
    pub quote_token: ERC20Token,
}

pub enum ValidatedTokenPair {
    ERC20ERC20(ERC20TokenPair),
    ETHERC20(ERC20Token),
    ERC20ETH(ERC20Token),
}

impl ValidatedTokenPair {
    pub fn new(
        base_token_index: TokenIndex,
        quote_token_index: TokenIndex,
        custom_erc20_list: &[Address],
    ) -> Result<Self, GoblinError> {
        require!(
            base_token_index != quote_token_index,
            GoblinError::InvalidTokenPair
        );

        let base_token = base_token_index.to_token(custom_erc20_list)?;
        let quote_token = quote_token_index.to_token(custom_erc20_list)?;

        match (base_token, quote_token) {
            (Token::ERC20(base_token), Token::ERC20(quote_token)) => {
                Ok(ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                    base_token,
                    quote_token,
                }))
            }
            (Token::Eth, Token::ERC20(quote_erc20)) => {
                Ok(ValidatedTokenPair::ETHERC20(quote_erc20))
            }
            (Token::ERC20(base_erc20), Token::Eth) => Ok(ValidatedTokenPair::ERC20ETH(base_erc20)),
            (Token::Eth, Token::Eth) => {
                // This case is unreachable due to the require! check above
                Err(GoblinError::InvalidTokenPair)
            }
        }
    }

    /// Update token stores for a maker upon a match
    ///
    /// # Arguments
    ///
    /// * `maker`- Maker address
    /// * `atoms`- The atoms lost by the taker. Add to the maker's free tokens.
    /// * `atoms_opposite`- The atoms gained by the maker. Subtract from maker's locked tokens.
    pub fn update_maker_stores<S>(
        &self,
        maker: &Address,
        atoms: S::Atoms,
        atoms_opposite: <S::Opposite as SideMarker>::Atoms,
    ) -> Result<(), GoblinError>
    where
        S: SideMarker,
    {
        match self {
            ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
                base_token,
                quote_token,
            }) => {
                let base_key = ERC20StoreKey::new(maker, base_token.address());
                let quote_key = ERC20StoreKey::new(maker, quote_token.address());

                let mut base_store = ERC20Store::load(&base_key);
                let mut quote_store = ERC20Store::load(&quote_key);

                S::update_maker_stores(
                    base_store.as_mut(),
                    quote_store.as_mut(),
                    atoms,
                    atoms_opposite,
                );

                base_store.as_ref().store(&base_key);
                quote_store.as_ref().store(&quote_key);
            }
            ValidatedTokenPair::ETHERC20(quote_token) => {
                let base_key = EthStoreKey::new(maker);
                let quote_key = ERC20StoreKey::new(maker, quote_token.address());

                let mut base_store = EthStore::load(&base_key);
                let mut quote_store = ERC20Store::load(&quote_key);

                S::update_maker_stores(
                    base_store.as_mut(),
                    quote_store.as_mut(),
                    atoms,
                    atoms_opposite,
                );

                base_store.as_ref().store(&base_key);
                quote_store.as_ref().store(&quote_key);
            }
            ValidatedTokenPair::ERC20ETH(base_token) => {
                let base_key = ERC20StoreKey::new(maker, base_token.address());
                let quote_key = EthStoreKey::new(maker);

                let mut base_store = ERC20Store::load(&base_key);
                let mut quote_store = EthStore::load(&quote_key);

                S::update_maker_stores(
                    base_store.as_mut(),
                    quote_store.as_mut(),
                    atoms,
                    atoms_opposite,
                );

                base_store.as_ref().store(&base_key);
                quote_store.as_ref().store(&quote_key);
            }
        }
        Ok(())
    }

    pub const fn discriminator(&self) -> u8 {
        match self {
            ValidatedTokenPair::ERC20ERC20(_) => 3,
            ValidatedTokenPair::ETHERC20(_) => 4,
            ValidatedTokenPair::ERC20ETH(_) => 5,
        }
    }
}
