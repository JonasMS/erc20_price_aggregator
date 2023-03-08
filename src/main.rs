use dotenv::dotenv;
use price_aggregator::config::get_config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config();

    price_aggregator::run(config).await;
}
