use crate::config::Config;
use crate::exchange::get_exchange_rates;
use crate::rate_query_factory::get_rate_queries;

pub mod config;
mod exchange;
mod rate_query_factory;

pub async fn run(config: Config) -> () {
    println!("EXECUTING run()");

    let rate_queries = get_rate_queries(&config.token_pairs);
    println!("Rate Queries: {:?}", rate_queries);

    let exchange_rates = get_exchange_rates(&rate_queries).await;

    println!("Exchange Rates: {:?}", exchange_rates);
}
