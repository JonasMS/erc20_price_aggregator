use crate::config::get_config;
use crate::exchange::get_exchange_rates;
use crate::rate_query_factory::get_rate_queries;
mod config;
mod rate_query_factory;
// mod exchange;

fn main() {
    let config = get_config();
    let rate_queries = get_rate_queries(&config.token_pairs);
    let exchange_rates = get_exchange_rates(&rate_queries);

    println!("Rate Queries: {:?}", rate_queries);
}
