use crate::config::{Pool, TokenPair};

#[derive(Debug)]
pub struct RateQuery {
    pub token_in: String, // TODO create Address type
    pub token_out: String,
    pub pool: Pool,
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
                    network_id: pool.network_id.clone(),
                    exchange_id: pool.exchange_id.clone(),
                    fee: Some(pool.fee.clone().unwrap_or(0.0)),
                },
            });
        }
    }

    rate_queries
}
