use crate::exchange::ExchangeRate;
use crate::rate_query_factory::RateQuery;
use std::io;

pub fn get_exchange_rate(rate_query: &RateQuery) -> Result<ExchangeRate, io::Error> {
    // Err(RateQueryError(
    //     rate_query,
    //     String::from("Some error message"),
    // ))

    Ok(ExchangeRate {
        query: rate_query,
        rate: 1000,
    })
}
