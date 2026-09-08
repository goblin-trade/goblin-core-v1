mod impl_fixed_decode;

pub struct HeaderFlags {
    /// Whether to read custom recipient address from payload
    pub read_custom_recipient: bool,

    /// Whether to read msg.value from hostio
    pub read_msg_value: bool,

    /// Whether to process dynamic markets
    pub process_dynamic_markets: bool,

    /// Whether to read ETH withdraw amount from args and withdraw ETH
    pub withdraw_eth: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,

    /// Number of custom ERC20 tokens to read
    pub custom_erc20_count: usize,
}
