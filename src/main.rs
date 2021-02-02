use chrono::prelude::*;
use coingecko::{Client, SimplePriceReq};
use colored::*;
use futures::future::join_all;
use math::round;
use rust_decimal::prelude::*;
use rusty_money::iso;
use std::collections::HashMap;
use std::error::Error;

#[path = "./currency.rs"]
mod currency;

#[path = "./table.rs"]
mod table;

#[path = "./config.rs"]
mod conf;
use config::Value;

// use log::debug;
// use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let config = ConfigBuilder::new()
    //     .set_target_level(LevelFilter::Trace)
    //     .build();
    // let _ = SimpleLogger::init(LevelFilter::Debug, config);

    // global config file
    let conf: HashMap<String, Value> = conf::get_config().try_into()?;

    // get general money settings
    let money_conf: HashMap<String, Value> = conf["money"].to_owned().try_into()?;
    let base_decimals: i8 = money_conf["round"].to_owned().try_into().unwrap_or(3);
    let base_curr: Vec<String> = money_conf["currency"]
        .to_owned()
        .try_into()
        .unwrap_or(vec!["USD".to_owned()]);
    let main_curr: String = base_curr[0].to_uppercase(); // iso::find needs uppercase
    let def_curr: &iso::Currency = iso::find(&main_curr).to_owned().unwrap_or(iso::USD);

    // make Vec with request futures
    let http = isahc::HttpClient::new()?;
    let client = Client::new(http);
    let mut futs = Vec::new();
    let mut already = Vec::new();
    let assets: Vec<HashMap<String, HashMap<String, String>>> =
        conf["assets"].to_owned().try_into()?;
    for i in &assets {
        let keyname = i.keys().nth(0).unwrap().to_string();
        match already.contains(&keyname) {
            true => continue,
            false => {
                already.push(keyname.to_owned());
                let r = SimplePriceReq::new(keyname, base_curr[0].to_owned())
                    .include_market_cap()
                    .include_24hr_vol()
                    .include_24hr_change()
                    .include_last_updated_at();
                futs.push(client.simple_price(r));
            }
        };
    }
    let mut total_net: f64 = 0.0;
    let mut c: usize = 0;
    let resolved = join_all(futs).await;
    // build table
    let mut table = table::get_skeleton(&conf, &base_curr[0]);
    for i in resolved {
        let dict = i?.to_owned();
        let ticker_key = dict.keys().into_iter().nth(0).unwrap();
        let ticker_info = &dict[ticker_key];
        let ticker_price: f64 = ticker_info["usd"].to_string().parse().unwrap();
        let ticker_market_cap_f64: f64 =
            ticker_info["usd_market_cap"].to_f64().unwrap() / 1_000_000.0 as f64;
        let ticker_market_cap: Decimal = Decimal::from_f64(ticker_market_cap_f64).unwrap();
        let ticker_vol_day_f64: f64 =
            ticker_info["usd_24h_vol"].to_f64().unwrap() / 1_000_000.0 as f64;
        let ticker_vol_day: Decimal = Decimal::from_f64(ticker_vol_day_f64).unwrap();

        let ticker_price_flux: f64 = ticker_info["usd_24h_change"].to_f64().unwrap();
        let price_flux_pretty = round::stochastic(ticker_price_flux, 2).to_string() + "%";
        // let pretty_col = match ticker_price_flux < 0.0 {
        //     true => comfy_table::Color::Red,
        //     false => comfy_table::Color::Green,
        // };

        let last_update = ticker_info["last_updated_at"].to_i64().unwrap();
        let ti = Utc.timestamp(last_update, 0);
        let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
        let ht = last_update_min.to_string() + &"m";
        // let pretty_time_col = match last_update_min {
        //     0..=2 => comfy_table::Color::Green,
        //     3..=5 => comfy_table::Color::Yellow,
        //     _ => comfy_table::Color::Red,
        // };
        for x in &assets {

            let asset_type = x.keys().into_iter().next().unwrap();
            // println!("{:#?} {}", x, asset_type);
            if &ticker_key == &asset_type {
                c += 1;

                // let nest_dict = x.values().into_iter().next().unwrap();
                let nest_dict = &x[asset_type];
                let desc: String = match nest_dict.get("desc") {
                    Some(i) => i.to_string(),
                    None => "".to_owned(), // Ok(i) => i,
                                           // Err(_e) => ""
                };

                let qty: f64 = nest_dict["amount"].to_string().parse()?;
                let asset_net: Decimal = Decimal::from_str(&(ticker_price * qty).to_string())?;
                total_net += &asset_net.to_f64().unwrap();
                let qty_rounded = round::half_up(qty, base_decimals);

                let values: Vec<String> = vec![
                    c.to_string(),
                    ticker_key.to_string(),
                    qty_rounded.to_string(),
                    desc.to_string(),
                    currency::to_fiat(asset_net, def_curr).to_string(),
                    currency::to_fiat(Decimal::from_str(&ticker_price.to_string())?, def_curr)
                        .to_string(),
                    currency::to_fiat(ticker_market_cap, def_curr).to_string() + &"M",
                    currency::to_fiat(ticker_vol_day, def_curr).to_string() + &"M",
                    price_flux_pretty.to_string(),
                    ht.to_string(),
                ];
                let tint: tint::Color = match nest_dict.get("tint") {
                    Some(i) => tint::Color::from(i),
                    None => tint::Color::from("#FFFFFF"), // Ok(i) => i,
                                           // Err(_e) => ""
                };
                let row = table::get_row(&conf, values, tint);
                table.add_row(row);
            }
        }
    }

    let net_str: &str = &total_net.to_string();
    let pretty_net: String = currency::to_fiat(Decimal::from_str(net_str)?, def_curr).to_string();
    println!(
        "[!] {4}\n{0}\n[{2}] Total Net Worth: {1}{3}",
        table,
        pretty_net.green(),
        // "=".green(),
        "=".green(),
        base_curr[0].yellow(),
        base_curr[0]
    );

    Ok(())
}
