use crate::exchange::ExchangeRate;
use crate::rate_query_factory::RateQuery;
use ethers::{
    contract::abigen,
    core::{
        abi::{AbiDecode, ParseError},
        types::{Address, Bytes},
    },
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

    let (sqrtPriceX96, a, b, c, d, e, f) = pool.slot_0().call().await?;

    println!("sqrtPriceX96: {}", sqrtPriceX96);

    // println!("RESULT: {}", result.await.unwrap());
    Ok(ExchangeRate {
        query: rate_query,
        rate: 1000,
    })
}
