use crate::exchange::ExchangeRate;
use crate::rate_query_factory::RateQuery;
use ethers::{
    contract::abigen,
    core::types::{Address, U256},
    providers::{Http, Provider},
};
use std::env;
use std::sync::Arc;

abigen!(
    IQuoterV2,
    r#"[
        struct QuoteExactInputSingleParams {address tokenIn; address tokenOut; uint256 amountIn; uint24 fee; uint160 sqrtPriceLimitX96;}
        function quoteExactInputSingle(QuoteExactInputSingleParams memory params) external returns (uint256 amountOut, uint160 sqrtPriceX96After, uint32 initializedTicksCrossed, uint256 gasEstimate)
    ]"#,
);

pub async fn get_exchange_rate(
    rate_query: &RateQuery,
) -> Result<ExchangeRate, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ETH_RPC_URL").unwrap();
    let client = Provider::<Http>::try_from(&rpc_url)?;
    let client = Arc::new(client);

    let quoter = IQuoterV2::new(
        "0x61fFE014bA17989E743c5F6cB21bF9697530B21e".parse::<Address>()?,
        Arc::clone(&client),
    );

    let params = i_quoter_v2::QuoteExactInputSingleParams {
        token_in: rate_query.token_in.address.parse::<Address>()?,
        token_out: rate_query.token_out.address.parse::<Address>()?,
        amount_in: U256::from(1 * (10_u64.pow(rate_query.token_in.decimals))),
        fee: rate_query.pool.fee.ok_or("Pool fee missing")?,
        sqrt_price_limit_x96: U256::zero(),
    };

    let (quote, _a, _b_, _gas_estimate) = quoter.quote_exact_input_single(params).call().await?;

    let quote = quote / (10_u32.pow(rate_query.token_out.decimals));

    Ok(ExchangeRate {
        query: rate_query,
        rate: quote,
    })
}
