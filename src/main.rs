use std::collections::HashMap;
use crate::config::get_config;
mod config;

#[derive(Debug)]
struct TokenPair {
    token_in: String,
    token_out: String,
    networks: Vec<u64>,
}

#[derive(Debug)]
struct Exchange {
    token_pairs: Vec<TokenPair>
}

fn main() {
    let config = get_config();

    /* Construct HashMap of Exchanges */
    let mut exchanges = HashMap::new();
    
    for exchange in config.exchanges {
        let mut exchange_token_pairs = Vec::new();

        for token_pair in &config.token_pairs {
                if token_pair.exchanges.contains(&exchange.id) {
                exchange_token_pairs.push(TokenPair {
                    token_in: token_pair.token_in.clone(),
                    token_out: token_pair.token_out.clone(),
                    networks: token_pair.networks.clone()
                });
            }
        }

        exchanges.insert(exchange.id, Exchange {
            token_pairs: exchange_token_pairs
        });
    }

    println!("EXCHANGES MAP: {:?}", exchanges.get(&0));
    
    // for every network
    // for every token
    // fetch and print price
}
