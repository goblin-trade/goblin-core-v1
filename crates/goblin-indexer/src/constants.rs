use std::{env, str::FromStr};

use alloy::{primitives::Address, transports::http::reqwest::Url};

pub fn rpc_url() -> Url {
    Url::from_str(env::var("ETH_RPC_URL").unwrap().as_str()).unwrap()
}

pub fn contract_address() -> Address {
    Address::from_str(env::var("CONTRACT").unwrap().as_str()).unwrap()
}
