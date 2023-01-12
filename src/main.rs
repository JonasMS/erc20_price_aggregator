use price_aggregator::config::get_config;

fn main() {
    let config = get_config();

    println!("CONFIG: {:?}", config);

    price_aggregator::run(config);
}
