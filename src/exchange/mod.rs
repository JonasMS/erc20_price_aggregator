use create::rate_query_factory::{Exchange, RateQuery};
use create::uniswap;

mod rate_query_factory;
mod uniswap;

pub struct ExchangeRate {
    pub query: RateQuery,
    pub rate: u64,
}

pub fn get_exchange_rates(rate_queries: &Vec<RateQuery>) -> ExchangeRate {
    let exchange_rates: Vec<ExchangeRate> = Vec::new();
    // for every rate query
    //  match on exchange
    for rate_query in rate_queries {
        match rate_query.pool.exchange {
            Exchange::Uniswap => exchange_rates.push(uniswap.get_exchange_rate(rate_query)),
            _ => None,
        }
    }
}
