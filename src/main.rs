// use clap::{Clap};
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


// #[derive(Debug)]
// struct CoingeckoResponse {
//     asset: String,
//     price: f64,
//     market_cap: f64,
//     day_vol: f64,
//     day_change: f64,
//     updated_at: i64, //
// }

// use log::debug;
// use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
// use clap::{App, ArgMatches, Arg};
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
    let mut total_net: f64 = 0.0; // used later to calculate and print total net worth

    // make Vec with request futures
    let http = isahc::HttpClient::new()?;
    let client = Client::new(http);
    let assets: Vec<HashMap<String, HashMap<String, String>>> =
    conf["assets"].to_owned().try_into()?;
    let mut futs = Vec::new();
    let mut already = Vec::new();
    for i in &assets {
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
    }
    let resolved = join_all(futs).await;

    let asset_vec : Vec<currency::Asset> = Vec::new();

    // build row data from requests
    let mut old_table = table::get_skeleton(&conf, &base_curr[0]);
    let mut row_data: Vec<Vec<String>> = Vec::new();
    for i in resolved {

        let dict = i.unwrap().to_owned();
        let req_info = currency::to_coingecko_response(&dict);
        println!("{:#?}", req_info);

        // let dict = i?.to_owned();
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
        let price_flux_pretty =
            round::stochastic(ticker_price_flux, base_decimals).to_string() + "%";

        let last_update = ticker_info["last_updated_at"].to_i64().unwrap();
        let ti = Utc.timestamp(last_update, 0);
        let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
        let ht = last_update_min.to_string() + &"m";

        // let asset_request = CoingeckoResponse {
        //     asset: ticker_key.to_owned(),
        //     price: ticker_price,
        //     market_cap: ticker_market_cap_f64,
        //     day_vol: ticker_vol_day_f64,
        //     day_change: ticker_price_flux,
        //     updated_at: last_update_min
        // };

        for x in &assets {
            let asset_type = x.keys().into_iter().next().unwrap();
            // println!("{:#?} {}", x, asset_type);
            // asset_vec.push(Asset
            //
            //     });
            if &ticker_key == &asset_type {
                // c += 1;

                // let nest_dict = x.values().into_iter().next().unwrap();
                let nest_dict = &x[asset_type];
                let desc: String = match nest_dict.get("desc") {
                    Some(i) => i.to_string(),
                    None => "".to_owned(), // Ok(i) => i,
                                           // Err(_e) => ""
                };
                // let descc : String = nest_dict.get("desc").or("").try_into();

                let qty: f64 = nest_dict["amount"].to_string().parse()?;
                let asset_net: Decimal = Decimal::from_str(&(ticker_price * qty).to_string())?;
                total_net += &asset_net.to_f64().unwrap();
                let qty_rounded = round::half_up(qty, base_decimals);

                let values: Vec<String> = vec![
                    // c.to_string(),
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
                let row_data_clone = values.clone();
                row_data.push(row_data_clone);


                // @TODO: get the tint in the sort section
                let tint: tint::Color = match nest_dict.get("tint") {
                    Some(i) => tint::Color::from(i),
                    None => tint::Color::from("#FFFFFF"), // Ok(i) => i,
                                                          // Err(_e) => ""
                };
                let row = table::get_row(&conf, 0, values, tint);
                old_table.add_row(row);


            }
        }
    }


    let mut table = table::get_skeleton(&conf, &base_curr[0]);
    // might want to collect all the values
    println!("Unsorted: {:?}\n", row_data);
    // this
    // row_data.sort_by(|a, b| b[0].cmp(&a[0]));
    // let sort_key =
    // let sort_inverse =
    let sorty = table::sort_rows(&conf, row_data);
    println!("\nSorted: {:?}\n", sorty,);
    // let mut c: usize = 0;
    for (i, item) in sorty.iter().enumerate() {

        let vec_row = item.to_owned();
        // let asset_name = vec_row[0];
        // println!("{:#?} {:#?}", vec_row[0], &assets[asset_name]);

        // let nest_dict = &assets[vec_row[0]];
        // let desc: String = match nest_dict.get("desc") {
        //     Some(i) => i.to_string(),
        //     None => "".to_owned(), // Ok(i) => i,
        //                            // Err(_e) => ""
        // };

        // let tint
        // let currency = assets.get(item[0]);
        let row = table::get_row(&conf, i+1, vec_row, tint::Color::from("green"));
        table.add_row(row);
    }

    // have a sort phase

    // then add_row from them all

    let net_str: &str = &total_net.to_string();
    let pretty_net: String = currency::to_fiat(Decimal::from_str(net_str)?, def_curr).to_string();
    println!(
        "{0}\n[{2}] Total Net Worth: {1}{3}",
        table,
        pretty_net.green(),
        "=".green(),
        base_curr[0].to_uppercase().yellow(),
    );

    Ok(())
}
