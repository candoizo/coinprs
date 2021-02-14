pub fn to_shortnum(n: f64) -> String {

    // let ne = n > 0.0;
    // let v = match n {
    //     0.0..=1e3 => {},
    //     1e3..=1e6 => {},
    //     1e6..=1e9 => {},
    //     _ => {}
    // };
    // v = match ne {
    //     true => v = v * 1.0,
    //     false => v = v * -1.0
    // };
    // if ne {
    //     v *= 1;
    // } else {
    //
    // }


    match n > 1e9 {
        true => (((n / 1e9) * 100.0).trunc() / 100.0).to_string() + &"B",
        false => match n > (1e6 - 1f64) {
            true => (((n / 1e6) * 100.0).trunc() / 100.0).to_string() + &"M",
            false => match n > (1e3 - 1f64) {
                true => (((n / 1e3) * 100.0).trunc() / 100.0).to_string() + &"K",
                false => n.to_string(),
            }
        }
    }
}

//
// use coingecko::{Client, SimplePriceReq, SimplePrices};
// use config::Value;
// use isahc::HttpClient;
// pub async fn get_price(http: HttpClient, i: Value) -> Result<SimplePrices, coingecko::Error> {
//     let t = i.into_table().unwrap();
//     let client = Client::new(http.to_owned());
//     let name: String = t["name"].to_string();
//     let req = SimplePriceReq::new(name, "usd".into())
//         .include_market_cap()
//         .include_24hr_vol()
//         .include_24hr_change()
//         .include_last_updated_at();
//     Ok(client.simple_price(req).await.unwrap())
// }

use rust_decimal::Decimal;
use rusty_money::iso::Currency;
use rusty_money::{iso, Money};
pub fn to_fiat(x: Decimal, fiat: &'static Currency) -> Money<'static, iso::Currency> {
    Money::from_decimal(x, *&fiat)
}

#[derive(Debug)]
pub struct Asset {
    pub name: String,
    pub desc: String,
    pub tint: String,
    pub amount: f64,
    pub decimals: usize,
}

pub fn to_asset(x: &HashMap<String, HashMap<String, String>>) -> Asset {

    let asset_type = x.keys().into_iter().next().unwrap();
    let info = &x[asset_type];

    let desc: String = match info.get("desc") {
        Some(i) => i.to_string(),
        None => "".to_owned(),
    };

    let tint: String = match info.get("tint") {
        Some(i) => i.to_string(),
        None => "".to_owned(),
    };

    let amount: f64 = match info.get("amount") {
        Some(i) => i.parse().unwrap_or(0.0),
        None => 0.0,
    };

    let decimals: usize = match info.get("decimals") {
        Some(i) => i.parse().unwrap_or(0),
        None => 2,
    };

    Asset {
        name: asset_type.to_string(),
        desc,
        tint,
        amount,
        decimals,
    }
}

#[derive(Debug)]
pub struct CoingeckoResponse {
    pub asset: String,
    pub price: f64,
    pub market_cap: f64,
    pub day_vol: f64,
    pub day_change: f64,
    pub updated_at: u64, // milliseconds since last update
}

// use chrono::prelude::*;
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
    let day_change: f64 = ticker_info["usd_24h_change"].to_f64().unwrap();
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

    let updated_at: u64 = ticker_info["last_updated_at"].to_u64().unwrap();
    // let ti = Utc.timestamp(last_update, 0);
    // let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
    // let ht = last_update_min.to_string() + &"m";

    CoingeckoResponse {
        asset: ticker_key,
        day_change,
        price,
        market_cap,
        day_vol,
        updated_at,
    }
}

use std::collections::HashMap;
// pub fn get_reqs(
//     assets: &Vec<HashMap<String, HashMap<String, String>>>,
//     curr: &Vec<String>,
// ) -> Vec<SimplePriceReq> {
//     let mut futs = Vec::new();
//     let mut already = Vec::new();
//     for i in assets {
//         let keyname = i.keys().next().unwrap().to_string();
//         if !already.contains(&keyname) {
//             {
//                 already.push(keyname.to_owned());
//                 let req_opts: SimplePriceReq =
//                     SimplePriceReq::new(keyname.to_owned(), curr[0].to_owned())
//                         .include_market_cap()
//                         .include_24hr_vol()
//                         .include_24hr_change()
//                         .include_last_updated_at()
//                         .into();
//                 // {
//                 futs.push(req_opts);
//                 // }
//             }
//         }
//     }
//     futs
// }
