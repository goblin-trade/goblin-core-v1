use crate::{
    markets::MarketVariant, settlement::SenderBalanceUpdates, tokens::HardcodedIndex, types::Pair,
};

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K>;

    /// Queue amounts to deposit
    fn deposit<M: MarketVariant>(
        sender_balance_updates: &mut SenderBalanceUpdates,
        index_pair: &Self::ResolvedPair<HardcodedIndex>,
        deposits_pair: Self::ResolvedPair<i64>,
    );
}

impl PairShape for Pair<ETH, ERC20> {
    const DISCRIMINATOR: u8 = 0;

    type ResolvedPair<K> = K;

    fn deposit<M: MarketVariant>(
        sender_balance_updates: &mut SenderBalanceUpdates,
        index_pair: &Self::ResolvedPair<HardcodedIndex>,
        deposits: Self::ResolvedPair<i64>,
    ) {
        let quote_delta = index_pair.token_delta_mut(sender_balance_updates);
        quote_delta.deposit(deposits);
    }
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K> = K;

    fn deposit<M: MarketVariant>(
        sender_balance_updates: &mut SenderBalanceUpdates,
        index_pair: &Self::ResolvedPair<HardcodedIndex>,
        deposits: Self::ResolvedPair<i64>,
    ) {
        let base_delta = index_pair.token_delta_mut(sender_balance_updates);
        base_delta.deposit(deposits);
    }
}

/// A pair of two ERC20 tokens.
/// The decoder guarantees that the two tokens are different.
impl PairShape for Pair<ERC20, ERC20> {
    const DISCRIMINATOR: u8 = 2;

    type ResolvedPair<K> = Pair<K, K>;

    fn deposit<M: MarketVariant>(
        sender_balance_updates: &mut SenderBalanceUpdates,
        index_pair: &Self::ResolvedPair<HardcodedIndex>,
        deposits: Self::ResolvedPair<i64>,
    ) {
        let base_delta = index_pair.base.token_delta_mut(sender_balance_updates);
        base_delta.deposit(deposits.base);

        let quote_delta = index_pair.quote.token_delta_mut(sender_balance_updates);
        quote_delta.deposit(deposits.quote);
    }
}
