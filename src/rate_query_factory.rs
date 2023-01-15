use crate::config::TokenPair;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Network {
    Ethereum,
    // Polygon,
    // Abitrum,
}

// TODO find simpler way to impl Display
impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ", &self)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Exchange {
    Uniswap,
    // Balancer,
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ", &self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenSymbol {
    WETH,
    USDC,
}

impl fmt::Display for TokenSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ", &self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub address: String,
    pub symbol: TokenSymbol,
    pub decimals: u64,
}

#[derive(Debug)]
pub struct Pool {
    pub network: Network,
    pub exchange: Exchange,
    pub address: Option<String>,
    pub fee: Option<f32>,
}

#[derive(Debug)]
pub struct RateQuery {
    pub id: String,
    pub token_in: Token, // TODO create Address type
    pub token_out: Token,
    pub pool: Pool,
}

fn get_token_symbol_to_address_map() -> HashMap<(TokenSymbol, Network), String> {
    let mut token_symbol_to_address_map = HashMap::new();

    token_symbol_to_address_map.insert(
        (TokenSymbol::WETH, Network::Ethereum),
        String::from("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"),
    );
    token_symbol_to_address_map.insert(
        (TokenSymbol::USDC, Network::Ethereum),
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

fn get_token(token_symbol: &str) -> TokenSymbol {
    match token_symbol {
        "WETH" => TokenSymbol::WETH,
        "USDC" => TokenSymbol::USDC,
        _ => panic!("Token Symbol didn't match: {}", token_symbol),
    }
}

/*
 * Given a list of TokenPairs, returns a list of RateQuery objects.
*/
pub fn get_rate_queries(token_pairs: &Vec<TokenPair>) -> Vec<RateQuery> {
    let mut rate_queries: Vec<RateQuery> = Vec::new();
    let token_symbol_to_address_map = get_token_symbol_to_address_map();

    // for each pool in a token pair
    // create a RateQuery
    for (idx, token_pair) in token_pairs.iter().enumerate() {
        for pool in &token_pair.pools {
            let network = get_network(&pool.network_id);
            let token_in = get_token(&token_pair.token_in.symbol);
            let token_out = get_token(&token_pair.token_out.symbol);

            rate_queries.push(RateQuery {
                id: idx.to_string(),
                token_in: Token {
                    address: token_symbol_to_address_map
                        .get(&(token_in, network))
                        .expect("`tokin_in` address not found")
                        .to_string(),
                    symbol: token_in.clone(),
                    decimals: token_pair.token_in.decimals.clone(),
                },
                token_out: Token {
                    address: token_symbol_to_address_map
                        .get(&(token_out, network))
                        .expect("`tokin_in` address not found")
                        .to_string(),
                    symbol: token_out.clone(),
                    decimals: token_pair.token_out.decimals.clone(),
                },
                pool: Pool {
                    network: get_network(&pool.network_id),
                    exchange: get_exchange(&pool.exchange_id),
                    address: pool.address.clone(),
                    fee: pool.fee.clone(),
                },
            });
        }
    }

    rate_queries
}
