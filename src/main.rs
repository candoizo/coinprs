#![feature(hash_drain_filter)]
#![allow(dead_code)]
use coingecko::{Client, SimplePriceReq};
use colored::*;
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
// use clap::{App, ArgMatches, Arg};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // let config = ConfigBuilder::new()
    //     .set_target_level(LevelFilter::Trace)
    //     .build();
    // let _ = SimpleLogger::init(LevelFilter::Debug, config);

    let conf_two: config::Config = conf::get_config();
    let conf: HashMap<String, Value> = conf::get_config().try_into()?;
    let _decimals: i8 = conf_two.get::<i8>("money.round").unwrap_or(2);
    let base_curr: Vec<String> = conf_two
        .get::<Vec<String>>("money.currency")
        .unwrap_or(vec!["USD".to_owned()]);
    let main_curr: String = base_curr[0].to_uppercase(); // iso::find needs uppercase
    let def_curr: &iso::Currency = iso::find(&main_curr).to_owned().unwrap_or(iso::USD);

    // make Vec with request futures
    let http = isahc::HttpClient::new()?;
    let client = Client::new(http);
    let assets: Vec<HashMap<String, HashMap<String, String>>> = conf_two
        .get::<Vec<HashMap<_, _>>>("assets")
        .unwrap_or(vec![HashMap::new()]);

    use indicatif::{ProgressBar, ProgressStyle};
    let bar = ProgressBar::new(assets.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.red/white}{pos:>7}/{len:7}{msg}")
            .progress_chars(">>="),
    );
    bar.set_message("Scanning.");

    let mut already = Vec::new();
    let mut futs = Vec::new();
    for (inc, i) in assets.iter().enumerate() {
        // println!("{} {}", inc, i);
        let keyname = i.keys().next().unwrap().to_string();
        if !already.contains(&keyname) {
            already.push(keyname.to_owned());
            let req = SimplePriceReq::new(keyname, base_curr[0].to_owned())
                .include_market_cap()
                .include_24hr_vol()
                .include_24hr_change()
                .include_last_updated_at();
            futs.push(client.simple_price(req));
        }
        bar.inc(1);
        // inc += 1;
        if inc == 3 {
            bar.set_message("Scanning..");
        } else if inc == 6 {
            bar.set_message("Scanning...");
        }
    }

    let resolved = futures_util::future::join_all(futs).await;
    let req_map: HashMap<String, currency::CoingeckoResponse> = resolved
        .iter()
        .map(|x| {
            let dict = match x {
                Ok(i) => i.to_owned(),
                Err(_i) => panic!("Network request error"),
            };
            let info = currency::to_coingecko_response(&dict);
            (info.asset.to_string(), info)
        })
        .collect();

    let mut total_net: f64 = 0.0; // calculate and print total net worth
    let mut btc_price : f64 = 0.0;
    let row_data: Vec<table::RowData> = assets
        .iter()
        .map(|x| {
            bar.inc(1);
            let asset: currency::Asset = currency::to_asset(x);
            let ticker_info = req_map.get(&asset.name).unwrap();
            // total_net += ticker_info.price * asset.amount;
            let values = table::RowData {
                tint: asset.tint,
                currency: asset.name,
                desc: asset.desc,
                amount: asset.amount,
                value: (ticker_info.price * asset.amount),
                price: ticker_info.price,
                market_cap: ticker_info.market_cap,
                day_vol: ticker_info.day_vol,
                day_change: ticker_info.day_change,
                updated_at: ticker_info.updated_at,
            };
            total_net += values.value;

            if btc_price < 1.0 {
                btc_price = match "bitcoin" == values.currency {
                    true => values.price,
                    false => 0.0
                }
            }
            // if assert_eq!("bitcoin", asset.name) {
            //     btc_price = values.price;
            // }

            values
        })
        .collect();
    bar.set_message("Done!");
    bar.finish();

    // let sorted = table::sort_rows(&conf_two, &conf, row_data);
    let mut table = table::get_skeleton(&conf_two, &conf, &base_curr[0]);
    for (index, item) in table::sort_rows(&conf_two, &conf, row_data)
        .iter()
        .enumerate()
    {
        let row = table::get_table_row(&conf_two, &conf, index, item);
        table.add_row(row);
    }

    let net_str: &str = &total_net.to_string();
    let btc_net : &str = &((total_net / btc_price as f64 * 1000f64).trunc() / 1000f64).to_string();
    let pretty_net: String = currency::to_fiat(Decimal::from_str(net_str)?, def_curr).to_string();

    println!(
        "{0}\n[{2}] USD Net Worth: {1}{3}\n[{2}] BTC Net Worth: {4}{5}",
        table,
        pretty_net.green(),
        "=".green(),
        main_curr.yellow(),
        btc_net.green(),
        "₿".yellow()

    );

    Ok(()) // 😍
}
