use crate::config::{Pool, TokenPair};
use std::collections::HashMap;

pub fn get_erc20_address_map() -> HashMap<(&'static str, i64), &'static str> {
    let mut erc20_addresses_map = HashMap::new();

    // Ethereum Mainnet
    erc20_addresses_map.insert(("USDC", 1), "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    erc20_addresses_map.insert(("WETH", 1), "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

    return erc20_addresses_map;
}

pub fn get_exchange_rates(token_pairs: Vec<TokenPair>) -> () {}
