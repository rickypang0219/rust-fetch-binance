use reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FuturePrice {
    timestamp: i64,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
    close_time: i64,
    quote_asset_volume: String,
    number_of_trades: i64,
    taker_buy_volume: String,
    taker_buy_quote_asset_volume: String,
    ignore: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let params = [
        ("pair", "BTCUSDT"),
        ("contractType", "PERPETUAL"),
        ("interval", "1m"),
        ("limit", "2"),
        ("StartTime", "2024-06-23"),
    ];

    let mut full_response = Vec::new();
    for _i in 0..2 {
        let response: Vec<FuturePrice> = client
            .get("https://fapi.binance.com/fapi/v1/continuousKlines")
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        // println!("Response: {:#?}", response);
        full_response.extend(response);
    }

    println!("{:#?}", full_response);

    Ok(())
}
