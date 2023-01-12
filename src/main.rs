use price_aggregator::config::get_config;

#[async_std::main]
async fn main() {
    let config = get_config();

    println!("CONFIG: {:?}", config);

    price_aggregator::run(config).await;
}
