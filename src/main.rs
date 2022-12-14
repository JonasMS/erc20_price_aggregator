use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Network {
    id: u64,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Exchange {
    address: String,
    networks: Vec<u64>,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct TokenPair {
    token_in: String,
    token_out: String,
    pool_address: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    networks: Vec<Network>,
    exchanges: Vec<Exchange>,
    token_pairs: Vec<TokenPair>,
}

fn main() {
    // read config file
    let config_data = fs::read_to_string("./config.json").expect("Failed to read config file.");
    let config: Config = serde_json::from_str(&config_data).expect("Failed to parse config.");

    println!(
        "NETWORK: {}, {}",
        config.networks[0].id, config.networks[0].name
    );

    // for every exchange
    // for every network
    // for every token
    // fetch and print price
}
