use crate::rate_query_factory::{Exchange, RateQuery};
// use std::fmt;

mod uniswap;

#[derive(Debug)]
pub struct ExchangeRate<'a> {
    pub query: &'a RateQuery,
    pub rate: u64,
}

pub async fn get_exchange_rates(rate_queries: &Vec<RateQuery>) -> Vec<ExchangeRate> {
    let mut exchange_rates: Vec<ExchangeRate> = Vec::new();
    // for every rate query
    //  match on exchange
    for rate_query in rate_queries {
        match rate_query.pool.exchange {
            Exchange::Uniswap => match uniswap::get_exchange_rate(rate_query).await {
                Ok(exchange_rate) => exchange_rates.push(exchange_rate),
                Err(E) => (), // TODO push error to list of errors, push errorful ExchangeRate to exchange_rates
            },
            _ => panic!("Invalid exchange: {:?}", rate_query.pool.exchange),
        }
    }

    exchange_rates
}

// type Result<T> = std::result::Result<T, RateQueryError>;

// struct RateQueryError(pub &RateQuery, pub String);

// impl fmt::Display for RateQueryError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?} {:?}", &self.0, &self.1)
//     }
// }
