use crate::config::TokenPair;

#[derive(Debug, Copy, Clone)]
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
    pub pool: Pool,
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

/*
 * Given a list of TokenPairs, returns a list of RateQuery objects.
*/
pub fn get_rate_queries(token_pairs: &Vec<TokenPair>) -> Vec<RateQuery> {
    let mut rate_queries: Vec<RateQuery> = Vec::new();

    // for each pool in a token pair
    // create a RateQuery
    for token_pair in token_pairs {
        for pool in &token_pair.pools {
            rate_queries.push(RateQuery {
                token_in: token_pair.token_in.clone(),
                token_out: token_pair.token_out.clone(),
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
