use std::env;

use ethers::types::Address;
use reqwest::{
    header::{self, HeaderMap},
    Client, ClientBuilder,
};

use crate::{types::{OrderV2, OrderRequest, OrderResponse, Order, OrderRequestV2, OrderResponseV2}, constants};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct OpenSeaApi {
    client: Client,
    base_url: String,
}

impl OpenSeaApi {
    pub fn new(cfg: OpenSeaApiConfig) -> Self {
        let mut builder = ClientBuilder::new();
        if let Some(api_key) = cfg.api_key {
            let mut headers = HeaderMap::new();
            headers.insert(
                "X-API-KEY",
                header::HeaderValue::from_str(&api_key).unwrap(),
            );
            builder = builder.default_headers(headers)
        }
        let client = builder.build().unwrap();

        Self {
            client,
            base_url: constants::API_BASE_URL.to_string(),
        }
    }

    pub async fn get_orders(&self, req: OrderRequest) -> Result<Vec<Order>, OpenSeaApiError> {
        let url = format!("{}/wyvern/v1/orders", self.base_url);

        // convert the request to a url encoded order
        let mut map = std::collections::HashMap::new();
        map.insert("side", serde_json::to_value(req.side)?);
        map.insert("token_id", serde_json::to_value(req.token_id)?);
        map.insert(
            "asset_contract_address",
            serde_json::to_value(req.contract_address)?,
        );
        map.insert("limit", serde_json::to_value(req.limit)?);

        let res = self.client.get(url).query(&map).send().await?;
        let text = res.text().await?;
        let resp: OrderResponse = serde_json::from_str(&text)?;

        Ok(resp.orders)
    }

    pub async fn get_order(&self, mut req: OrderRequest) -> Result<Order, OpenSeaApiError> {
        req.limit = 1;
        let res = self.get_orders(req.clone()).await?;
        let order = res
            .into_iter()
            .next()
            .ok_or(OpenSeaApiError::OrderNotFound {
                contract: req.contract_address,
                id: req.token_id,
            })?;
        Ok(order)
    }

    pub async fn get_order_v2(&self, req: OrderRequestV2) -> Result<OrderV2, OpenSeaApiError> {
        let url = format!("{}/orders/chain/{}/protocol/0x00000000000000adc04c56bf30ac9d3c0aaf14dc/{}", self.base_url, req.chain, req.order_hash);

        let res = self.client.get(url).send().await?;
        let text = res.text().await?;
        let resp: OrderResponseV2 = serde_json::from_str(&text)?;

        Ok(resp.order)
    }
}

#[derive(Clone, Debug)]
pub struct OpenSeaApiConfig {
    pub api_key: Option<String>,
}

impl Default for OpenSeaApiConfig {
    fn default() -> Self {
        Self {
            api_key: Some(env::var("OPENSEA_API_KEY").unwrap()),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpenSeaApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Order not found (token: {contract}, id: {id}")]
    OrderNotFound { contract: Address, id: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_get_order() {
        let api = OpenSeaApi::new(OpenSeaApiConfig::default());

        let req = OrderRequestV2 {
            chain: "ethereum".to_string(),
            order_hash: "0x5168ae982c8d0bd40267a318cf88c542b9e2dc1a8f73a655a20b987fc01f0cea"
                .parse()
                .unwrap(),
        };
        let order = api.get_order_v2(req).await.unwrap();
        println!("order: {:?}", &order);
    }
}
