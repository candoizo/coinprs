use chrono::prelude::*;
use coingecko::{Client, SimplePriceReq};
use colored::*;
use comfy_table::*;
use futures::future::join_all;
// use futures::prelude::*;
// use futures::try_join;
use math::round;
use rust_decimal::prelude::*;
use rusty_money::iso;
use std::collections::HashMap;
use std::error::Error;
// use std::str::FromStr;

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

    // get configs -> mappable dict
    let settings = conf::get_config();
    let conf = settings.try_into::<HashMap<String, Value>>()?;

    let base_decimals = conf["decimals"].to_owned().into_int()?;
    let mut base_curr = conf["base_currency"].to_string().to_uppercase();
    let def_curr = iso::find(base_curr.as_str()).to_owned().unwrap();
    let mut table = table::get_skeleton(&conf, &base_curr);
    let assets = conf["assets"].to_owned().into_array()?;

    // make Vec with request futures
    let http = isahc::HttpClient::new()?;
    let client = Client::new(http);
    let mut futs = Vec::new();
    let mut already = Vec::new();
    for i in &assets {
        let dict = i.to_owned().into_table()?;
        let name: String = dict.keys().nth(0).unwrap().to_string();
        // println!("{:?} {:?}", dict, name);
        if already.contains(&name) {
            // println!("already getting this ticker");
            continue;
        }
        already.push(name.to_string());
        let r = SimplePriceReq::new(name.to_owned(), base_curr.to_owned())
            .include_market_cap()
            .include_24hr_vol()
            .include_24hr_change()
            .include_last_updated_at();
        futs.push(client.simple_price(r));
    }
    let resolved = join_all(futs).await;

    let mut total_net: f64 = 0.0;
    let mut c: usize = 0;
    // for each price
    for i in resolved {
        let dict = i?.to_owned();
        let ticker_key = dict.keys().into_iter().nth(0).unwrap();
        let ticker_info = &dict[ticker_key];
        let ticker_price: f64 = ticker_info["usd"].to_string().parse().unwrap();
        let ticker_market_cap_f64: f64 = ticker_info["usd_market_cap"].to_f64().unwrap() / 1_000_000.0 as f64;
        let ticker_market_cap: Decimal = Decimal::from_f64(ticker_market_cap_f64).unwrap();
        let ticker_vol_day_f64: f64 = ticker_info["usd_24h_vol"].to_f64().unwrap() / 1_000_000.0 as f64;
        let ticker_vol_day: Decimal = Decimal::from_f64(ticker_vol_day_f64).unwrap();

        let ticker_price_flux: f64 = ticker_info["usd_24h_change"].to_f64().unwrap();
        let price_flux_pretty = round::stochastic(ticker_price_flux, 2).to_string() + "%";
        let mut pretty_col = comfy_table::Color::Green;
        if ticker_price_flux < 0.0 {
            pretty_col = comfy_table::Color::Red;
        }

        let last_update = ticker_info["last_updated_at"].to_i64().unwrap();
        let ti = Utc.timestamp(last_update, 0);
        // let ht = chrono_humanize::HumanTime::from(ti);
        let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
        let ht = last_update_min.to_string() + &"m";
        let mut pretty_time_col = comfy_table::Color::Green;
        if last_update_min > 2 {
            pretty_time_col = comfy_table::Color::Yellow;
        }
        if last_update_min > 4 {
            pretty_time_col = comfy_table::Color::Red;
        }

        for x in &assets {
            // for each asset that has a key = ticker_key, add row
            let asset_dict = x.to_owned().into_table()?;
            let asset_type = asset_dict.keys().into_iter().next().unwrap();

            if ticker_key == asset_type {
                c += 1;

                let nest_dict = asset_dict[ticker_key].to_owned().into_table()?;
                let desc: String = match nest_dict.get("desc") {
                    Some(i) => i.to_string(),
                    None => "".to_owned(), // Ok(i) => i,
                                           // Err(_e) => ""
                };

                // let tint : tint::Color = match nest_dict.get("tint") {
                //     Some(i) => tint::Color::from_hex(&i.to_string()),
                //     None => tint::Color::from_hex(&"#FFFFFF"),
                //     // Ok(i) => i,
                //     // Err(_e) => ""
                // };

                let qty: f64 = nest_dict["amount"].to_string().parse()?;
                let asset_net: Decimal = Decimal::from_str(&(ticker_price * qty).to_string())?;
                total_net += &asset_net.to_f64().unwrap();
                let qty_rounded = round::half_up(qty, base_decimals as i8);
                table.add_row(vec![
                    Cell::new(c.to_string()),
                    Cell::new(ticker_key.to_string()).add_attribute(Attribute::Bold),
                    Cell::new(qty_rounded.to_string()).fg(comfy_table::Color::Yellow),
                    Cell::new(desc.to_string()),
                    Cell::new(currency::to_fiat(asset_net, def_curr).to_string()),
                    Cell::new(
                        currency::to_fiat(Decimal::from_str(&ticker_price.to_string())?, def_curr)
                            .to_string(),
                    ),
                    Cell::new(currency::to_fiat(ticker_market_cap, def_curr).to_string() + &"M"),
                    Cell::new(currency::to_fiat(ticker_vol_day, def_curr).to_string() + &"M"),
                    Cell::new(price_flux_pretty.to_owned())
                        .fg(pretty_col)
                        .set_alignment(CellAlignment::Center),
                    Cell::new(ht.to_string())
                        .fg(pretty_time_col)
                        .set_alignment(CellAlignment::Center),
                ]);
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
        base_curr.yellow(),
        conf["base_currency"].to_string()
    );

    Ok(())
}
