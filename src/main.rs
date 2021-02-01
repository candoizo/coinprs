use chrono::prelude::*;
use coingecko::{Client, SimplePriceReq};
use colored::*;
use comfy_table::*;
use futures::future::join_all;
use futures::prelude::*;
use futures::try_join;
use math::round;
use rust_decimal::prelude::*;
use rusty_money::iso;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[path = "./currency.rs"]
mod currency;

#[path = "./table.rs"]
mod table;

#[path = "./config.rs"]
mod conf;
use config::Value;

use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    Ok(())
}
