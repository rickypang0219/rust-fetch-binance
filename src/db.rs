use polars::prelude::{df, PolarsError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuturePrice {
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

pub fn convert_futures_data_dataframe(
    future_data: Vec<FuturePrice>,
) -> Result<polars::prelude::DataFrame, PolarsError> {
    let timestamp: Vec<i64> = future_data.iter().map(|f| f.timestamp.clone()).collect();
    let open: Vec<f64> = future_data
        .iter()
        .map(|f| f.open.clone().parse::<f64>().unwrap())
        .collect();
    let high: Vec<f64> = future_data
        .iter()
        .map(|f| f.high.clone().parse::<f64>().unwrap())
        .collect();
    let low: Vec<f64> = future_data
        .iter()
        .map(|f| f.low.clone().parse::<f64>().unwrap())
        .collect();
    let close: Vec<f64> = future_data
        .iter()
        .map(|f| f.close.clone().parse::<f64>().unwrap())
        .collect();
    let volume: Vec<f64> = future_data
        .iter()
        .map(|f| f.volume.clone().parse::<f64>().unwrap())
        .collect();
    let close_time: Vec<i64> = future_data.iter().map(|f| f.close_time.clone()).collect();
    let quote_asset_volume: Vec<f64> = future_data
        .iter()
        .map(|f| f.volume.clone().parse::<f64>().unwrap())
        .collect();
    let number_of_trades: Vec<i64> = future_data
        .iter()
        .map(|f| f.number_of_trades.clone())
        .collect();
    let taker_buy_volume: Vec<f64> = future_data
        .iter()
        .map(|f| f.taker_buy_volume.clone().parse::<f64>().unwrap())
        .collect();
    let taker_buy_quote_asset_volume: Vec<f64> = future_data
        .iter()
        .map(|f| {
            f.taker_buy_quote_asset_volume
                .clone()
                .parse::<f64>()
                .unwrap()
        })
        .collect();
    df!(
        "timestamp" => &timestamp,
        "open" => &open,
        "high" => &high,
        "low" => &low,
        "close" => &close,
        "volume" => &volume,
        "close_time" =>close_time,
        "quote_asset_volume" => &quote_asset_volume,
        "number_of_trade" => &number_of_trades,
        "taker_buy_volume" => &taker_buy_volume,
        "taker_buy_quote_asset_volume" => &taker_buy_quote_asset_volume,
    )
}
