use price_aggregator::config::get_config;

fn main() {
    let config = get_config();

    println!("CONFIG: {:?}", config);

    price_aggregator::run(config);
    // let rate_queries = get_rate_queries(&config.token_pairs);
    // let exchange_rates = get_exchange_rates(&rate_queries).await;

    // println!("Rate Queries: {:?}", rate_queries);
    // println!("Exchange Rates: {:?}", exchange_rates);
}
