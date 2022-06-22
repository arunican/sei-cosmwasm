use cosmwasm_std::{QuerierWrapper, StdResult, QueryRequest};

use crate::query::{
    ExchangeRatesResponse,
    SeiQuery, SeiQueryWrapper,
};
use crate::route::SeiRoute;

/// This is a helper wrapper to easily use our custom queries
pub struct SeiQuerier<'a> {
    querier: &'a QuerierWrapper<'a, SeiQueryWrapper>,
}

impl<'a> SeiQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<SeiQueryWrapper>) -> Self {
        SeiQuerier { querier }
    }

    pub fn query_exchange_rates(&self) -> StdResult<ExchangeRatesResponse> {
        let request = QueryRequest::Custom(SeiQueryWrapper{
            route: SeiRoute::Oracle,
            query_data: SeiQuery::ExchangeRates {},
        });

        self.querier.query(&request)
    }
}