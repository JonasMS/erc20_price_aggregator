use crate::rate_query_factory::{Exchange, RateQuery};
use ethers::core::types::U256;
// use std::fmt;
use std::error::Error;

mod uniswap;

#[derive(Debug)]
pub struct ExchangeRate<'a> {
    pub query: &'a RateQuery,
    pub rate: U256,
}

pub async fn get_exchange_rates(
    rate_queries: &Vec<RateQuery>,
) -> (Vec<ExchangeRate>, Vec<Box<dyn Error>>) {
    let mut exchange_rates: Vec<ExchangeRate> = Vec::new();
    let mut errors: Vec<Box<dyn Error>> = Vec::new();
    // for every rate query
    //  match on exchange
    for rate_query in rate_queries {
        match rate_query.pool.exchange {
            Exchange::Uniswap => match uniswap::get_exchange_rate(rate_query).await {
                Ok(exchange_rate) => exchange_rates.push(exchange_rate),
                Err(err) => errors.push(err), // TODO push error to list of errors, push errorful ExchangeRate to exchange_rates
            },
            // _ => panic!("Invalid exchange: {:?}", rate_query.pool.exchange),
        }
    }

    (exchange_rates, errors)
}

// type Result<T> = std::result::Result<T, RateQueryError>;

// struct RateQueryError(pub &RateQuery, pub String);

// impl fmt::Display for RateQueryError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?} {:?}", &self.0, &self.1)
//     }
// }
