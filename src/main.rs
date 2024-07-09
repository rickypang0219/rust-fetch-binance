use reqwest::Error;
mod db;
mod scraper;
use db::{convert_futures_data_dataframe, FuturePrice};
// use scraper::fetch_futures_data;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let response = fetch_futures_data().await?;
    let response = FuturePrice::fetch().await?;
    let futures_df = convert_futures_data_dataframe(&response);
    Ok(println!("{:#?}", &futures_df))
}
