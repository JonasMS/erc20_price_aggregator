use crate::config::get_config;
use crate::rate_query_factory::get_rate_queries;
// use crate::exchange::get_erc20_address;
mod config;
mod rate_query_factory;
// mod exchange;

fn main() {
    // let (config, erc20_addresses_map) = get_config();
    let config = get_config();
    let rate_queries = get_rate_queries(&config.token_pairs);

    println!("Rate Queries: {:?}", rate_queries);
}
