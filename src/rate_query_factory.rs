use crate::config::TokenPair;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Network {
    Ethereum,
    Polygon,
    Abitrum,
}
#[derive(Debug, Copy, Clone)]
pub enum Exchange {
    Uniswap,
    Balancer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    WETH,
    USDC,
}

#[derive(Debug)]
pub struct Pool {
    pub network: Network,
    pub exchange: Exchange,
    pub fee: Option<f32>,
}

#[derive(Debug)]
pub struct RateQuery {
    pub token_in: String, // TODO create Address type
    pub token_out: String,
    pub token_in_symbol: String,
    pub token_out_symbol: String,
    pub pool: Pool,
}

fn get_token_symbol_to_address_map() -> HashMap<(Token, Network), String> {
    let mut token_symbol_to_address_map = HashMap::new();

    token_symbol_to_address_map.insert(
        (Token::WETH, Network::Ethereum),
        String::from("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
    );
    token_symbol_to_address_map.insert(
        (Token::USDC, Network::Ethereum),
        String::from("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
    );

    token_symbol_to_address_map
}

fn get_network(network_id: &u64) -> Network {
    match network_id {
        1 => Network::Ethereum,
        _ => panic!("Network ID didn't match: {}", network_id),
    }
}

fn get_exchange(exchange_id: &u64) -> Exchange {
    match exchange_id {
        0 => Exchange::Uniswap,
        _ => panic!("Exchange ID didn't match: {}", exchange_id),
    }
}

fn get_token(token_symbol: &str) -> Token {
    match token_symbol {
        "WETH" => Token::WETH,
        "USDC" => Token::USDC,
        _ => panic!("Token Symbol didn't match: {}", token_symbol),
    }
}

/*
 * Given a list of TokenPairs, returns a list of RateQuery objects.
*/
pub fn get_rate_queries(token_pairs: &Vec<TokenPair>) -> Vec<RateQuery> {
    let mut rate_queries: Vec<RateQuery> = Vec::new();
    let mut token_symbol_to_address_map = get_token_symbol_to_address_map();

    // for each pool in a token pair
    // create a RateQuery
    for token_pair in token_pairs {
        for pool in &token_pair.pools {
            let network = get_network(&pool.network_id);
            let token_in = get_token(&token_pair.token_in);
            let token_out = get_token(&token_pair.token_out);

            rate_queries.push(RateQuery {
                token_in_symbol: token_pair.token_in.clone(),
                token_out_symbol: token_pair.token_out.clone(),
                token_in: token_symbol_to_address_map
                    .get(&(token_in, network))
                    .expect("`tokin_in` address not found")
                    .to_string(),
                token_out: token_symbol_to_address_map
                    .get(&(token_out, network))
                    .expect("`tokin_out` address not found")
                    .to_string(),
                pool: Pool {
                    network: get_network(&pool.network_id),
                    exchange: get_exchange(&pool.exchange_id),
                    fee: Some(pool.fee.clone().unwrap_or(0.0)),
                },
            });
        }
    }

    rate_queries
}
