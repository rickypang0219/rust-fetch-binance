use reqwest;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let params = [
        ("pair", "BTCUSDT"),
        ("contractType", "PERPETUAL"),
        ("interval", "1m"),
    ];
    let response = client
        .get("https://fapi.binance.com/fapi/v1/continuousKlines")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    println!("Response: {:#?}", response);

    Ok(())
}
