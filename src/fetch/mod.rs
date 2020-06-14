use std::collections::HashMap;
use serde::Deserialize;
use reqwest;

use crate::parser;
use crate::codes;

// struct visibility defaults to private.
#[derive(Deserialize, Debug)]
pub struct RatesResponse {
    pub base: String,
    pub date: String,
    pub time_last_updated: i32,
    pub rates: HashMap<String, f32>
}

pub async fn rates(opts: &parser::Opt) -> Result<RatesResponse, reqwest::Error> {
    let url = ["https://api.exchangerate-api.com/v4/latest/",  &opts.currency_code.to_string()].concat();
    if opts.debug {
        println!("fetching from {}", &url);
    }
    let resp = reqwest::get(&url).await?;
    let rates = resp.json::<RatesResponse>().await?;
    Ok(rates)
}