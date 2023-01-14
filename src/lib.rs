use crate::config::Config;
use crate::console_display::print_rate_table;
use crate::exchange::get_exchange_rates;
use crate::rate_query_factory::get_rate_queries;

pub mod config;
mod console_display;
mod exchange;
mod rate_query_factory;

#[macro_use]
extern crate prettytable;

pub async fn run(config: Config) -> () {
    println!("EXECUTING run()");

    let rate_queries = get_rate_queries(&config.token_pairs);
    println!("RATE QUERIES: {:?}", rate_queries);

    let (exchange_rates, errors) = get_exchange_rates(&rate_queries).await;

    // println!("EXCHANGE RATES: {:?}", exchange_rates);
    // println!("ERRORS: {:?}", errors);
    print_rate_table(exchange_rates);
}
