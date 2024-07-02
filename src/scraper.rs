use crate::db::FuturePrice;
use reqwest;
use reqwest::Error;

pub async fn fetch_futures_data() -> Result<Vec<FuturePrice>, Error> {
    let client = reqwest::Client::new();
    let params = [
        ("pair", "BTCUSDT"),
        ("contractType", "PERPETUAL"),
        ("interval", "1m"),
        ("limit", "1500"),
        ("StartTime", "2024-06-23"),
    ];

    let response: Vec<FuturePrice> = client
        .get("https://fapi.binance.com/fapi/v1/continuousKlines")
        .query(&params)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
