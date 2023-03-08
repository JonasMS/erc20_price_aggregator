use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

// #[derive(Serialize, Deserialize)]
// pub struct Network {
//     pub id: u64,
//     pub name: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Exchange {
//     pub id: u64,
//     pub address: String,
//     pub networks: Vec<u64>,
//     pub name: String,
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    pub network_id: u64,
    pub exchange_id: u64,
    pub address: Option<String>,
    pub fee: Option<u32>,
}

// TODO create Address type
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub symbol: String,
    pub decimals: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPair {
    pub token_in: Token,
    pub token_out: Token,
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // pub networks: Vec<Network>,
    // pub exchanges: Vec<Exchange>,
    pub token_pairs: Vec<TokenPair>,
}

pub fn get_config() -> Config {
    let config_data = fs::read_to_string("./config.json").expect("Failed to read config file.");
    let config: Config = serde_json::from_str(&config_data).expect("Failed to parse config.");
    config
}
