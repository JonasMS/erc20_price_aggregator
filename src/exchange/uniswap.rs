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
    IUniswapV3Pool,
    r#"[
        function slot0() external view returns (uint160 sqrtPriceX96, int24 tick, uint16 observationIndex, uint16 observationCardinality, uint16 observationCardinalityNext, uint8 feeProtocol, bool unlocked)
    ]"#,
);

pub async fn get_exchange_rate(
    rate_query: &RateQuery,
) -> Result<ExchangeRate, Box<dyn std::error::Error>> {
    let rpc_url = env::var("ETH_RPC_URL").unwrap();
    let client = Provider::<Http>::try_from(&rpc_url)?;
    let client = Arc::new(client);

    let address = "0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8".parse::<Address>()?;
    let pool = IUniswapV3Pool::new(address, Arc::clone(&client));

    let (sqrt_price_x96, _a, _b, _c, _d, _e, _f) = pool.slot_0().call().await?;

    let decimal_difference = rate_query
        .token_in
        .decimals
        .abs_diff(rate_query.token_out.decimals);
    let decimal_difference = U256([decimal_difference; 4]);

    let p = sqrt_price_x96
        .checked_div(
            U256([2; 4])
                .checked_pow(U256([96; 4]))
                .ok_or("Error getting p denominator")?,
        )
        .ok_or("Error getting p")?;

    let price = U256([1; 4])
        .checked_div(p)
        .ok_or("Error getting 1/p")?
        .checked_mul(
            U256([10; 4])
                .checked_pow(decimal_difference)
                .ok_or("Error getting 2 ** decimal_difference")?,
        )
        .ok_or("Error multiplying by decimal difference")?;
    // * (2 * *(rate_query.token_in.decimals - rate_query.token_out.decimals).abs());

    println!("SQRT_PRICE_X96: {}", sqrt_price_x96);
    println!("PRICE: {}", price);

    // println!("RESULT: {}", result.await.unwrap());
    Ok(ExchangeRate {
        query: rate_query,
        rate: price,
    })
}
