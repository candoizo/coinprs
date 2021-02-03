use coingecko::{Client, SimplePriceReq, SimplePrices};
use config::Value;
use isahc::HttpClient;
pub async fn get_price(http: HttpClient, i: Value) -> Result<SimplePrices, coingecko::Error> {
    let t = i.into_table().unwrap();
    let client = Client::new(http.to_owned());
    let name: String = t["name"].to_string();
    let req = SimplePriceReq::new(name, "usd".into())
        .include_market_cap()
        .include_24hr_vol()
        .include_24hr_change()
        .include_last_updated_at();
    Ok(client.simple_price(req).await.unwrap())
}

use rust_decimal::Decimal;
use rusty_money::iso::Currency;
use rusty_money::{iso, Money};
pub fn to_fiat(x: Decimal, fiat: &'static Currency) -> Money<'static, iso::Currency> {
    Money::from_decimal(x, *&fiat)
}

#[derive(Debug)]
pub struct CoingeckoResponse {
    asset: String,
    price: f64,
    market_cap: f64,
    day_vol: f64,
    day_change: f64,
    updated_at: u64, // milliseconds since last update
}


#[derive(Debug)]
pub struct Asset {
    name: String,
    desc: String,
    amount: f64,
    tint: String,
    decimals: usize
}

use chrono::prelude::*;
use rust_decimal::prelude::ToPrimitive;
pub fn to_coingecko_response(
    x: &HashMap<String, HashMap<String, rust_decimal::Decimal>>,
) -> CoingeckoResponse {
    let dict = x.to_owned();
    let ticker_key: String = dict.keys().into_iter().nth(0).unwrap().to_string();
    let ticker_info: &HashMap<String, rust_decimal::Decimal> = &dict[&ticker_key];

    let price: f64 = ticker_info["usd"].to_f64().unwrap();
    let market_cap: f64 = ticker_info["usd_market_cap"].to_f64().unwrap();
    let day_vol: f64 = ticker_info["usd_24h_vol"].to_f64().unwrap();
    let day_flux: f64 = ticker_info["usd_24h_change"].to_f64().unwrap();
    // let info = ;
    // let ticker_price: f64 = ticker_info["usd"].to_string().parse().unwrap();
    // let ticker_market_cap_f64: f64 =
    // ticker_info["usd_market_cap"].to_f64().unwrap() / 1_000_000.0 as f64;
    // let ticker_market_cap: Decimal = Decimal::from_f64(ticker_market_cap_f64).unwrap();
    // let ticker_vol_day_f64: f64 =
    // ticker_info["usd_24h_vol"].to_f64().unwrap() / 1_000_000.0 as f64;
    // let ticker_vol_day: Decimal = Decimal::from_f64(ticker_vol_day_f64).unwrap();
    //
    // let ticker_price_flux: f64 = ticker_info["usd_24h_change"].to_f64().unwrap();
    // let price_flux_pretty =
    //     round::stochastic(ticker_price_flux, base_decimals).to_string() + "%";

    let last_update: u64 = ticker_info["last_updated_at"].to_u64().unwrap();
    // let ti = Utc.timestamp(last_update, 0);
    // let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
    // let ht = last_update_min.to_string() + &"m";

    CoingeckoResponse {
        asset: ticker_key,
        price: price,
        market_cap: market_cap,
        day_vol: day_vol,
        day_change: day_flux,
        updated_at: last_update,
    }
}

use std::collections::HashMap;
// pub async fn get_prices(client: Client, assets: Vec<HashMap<String, HashMap<String, String>>>, curr: Vec<String>) -> Vec<impl futures::Future> {
//     // let http = HttpClient::new().unwrap();
//     // let client = Client::new(http);
//     let mut futs = Vec::new();
//     let mut already = Vec::new();
//     // let assets: Vec<HashMap<String, HashMap<String, String>>> =
//     //     conf["assets"].to_owned().try_into()?;
//     for i in &assets {
//         let keyname = i.keys().next().unwrap().to_string();
//         if !already.contains(&keyname) {
//             already.push(keyname.to_owned());
//             let req_opts = SimplePriceReq::new(keyname, curr[0].to_owned())
//                 .include_market_cap()
//                 .include_24hr_vol()
//                 .include_24hr_change()
//                 .include_last_updated_at();
//             let req = client.simple_price(req_opts);
//             futs.push(req);
//         }
//     }
//     futs
// }

pub fn get_reqs(
    assets: Vec<HashMap<String, HashMap<String, String>>>,
    curr: &Vec<String>,
) -> Vec<SimplePriceReq> {
    let mut futs = Vec::new();
    let mut already = Vec::new();
    for i in assets {
        let keyname = i.keys().next().unwrap().to_string();
        if !already.contains(&keyname) {
            {
                already.push(keyname.to_owned());
                let req_opts: SimplePriceReq =
                    SimplePriceReq::new(keyname.to_owned(), curr[0].to_owned())
                        .include_market_cap()
                        .include_24hr_vol()
                        .include_24hr_change()
                        .include_last_updated_at()
                        .into();
                // {
                futs.push(req_opts);
                // }
            }
        }
    }
    futs
}
