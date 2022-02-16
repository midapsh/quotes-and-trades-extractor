use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    id: String,
    community_id: u32,
    name: String,
    native_token_id: String,
    logo_url: String,
    wrapped_token_id: String,
    usd_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalBalanceResponse {
    total_usd_value: f64,
    pub chain_list: Vec<Balance>,
}

pub async fn get_debank_balance_service(
    wallet: String,
) -> Result<Option<TotalBalanceResponse>, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https://openapi.debank.com/v1/user/total_balance")?;
    url.query_pairs_mut().append_pair("id", wallet.as_str());

    let resp = reqwest::get(url.as_str())
        .await?
        .json::<Option<TotalBalanceResponse>>()
        .await?;
    Ok(resp)
}
