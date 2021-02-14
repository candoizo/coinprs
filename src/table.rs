#![allow(dead_code)]
use chrono::prelude::*;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
#[path = "./currency.rs"]
mod currency;

#[path = "./config.rs"]
mod conf;
use config::Value;
use rust_decimal::prelude::FromPrimitive;
use std::collections::HashMap;
// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// use serde::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

// @TODO
// #[derive(Debug)]
// pub struct RowPrefs {
//     align: comfy_table::CellAlignment,
//     tint: tint::Color,
//     text: String
// }

#[derive(Debug, PartialEq, PartialOrd, Serialize)]
pub struct RowData {
    // pub prefs: RowPrefs
    // asset: self::currency::Asset,
    // info: self::currency::CoingeckoResponse,
    pub tint: String,
    pub currency: String,
    pub desc: String,
    pub amount: f64,
    pub price: f64,
    pub value: f64,
    pub market_cap: f64,
    pub day_vol: f64,
    pub day_change: f64,
    pub updated_at: u64,
}

fn get_field_by_name<T, R>(data: T, field: &str) -> R
where
    T: Serialize,
    R: DeserializeOwned,
{
    println!("{:?}", field);

    let mut map = match serde_value::to_value(data) {
        Ok(serde_value::Value::Map(map)) => map,
        _ => panic!("expected a struct"),
    };

    let key = serde_value::Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value,
        None => panic!("no such field"),
    };

    match R::deserialize(value) {
        Ok(r) => r,
        Err(_) => panic!("wrong type?"),
    }
}

// impl RowData {
//     pub fn get(val: String) -> String {
//         match val.as_str() {
//             "num" => b.num,
//             "currency" => b.name.cmp(&a.name),
//             "quantity" => b.name.cmp(&a.name),
//             "desc" => b.name.cmp(&a.name),
//             "price" => b.name.cmp(&a.name),
//             "value" => b.name.cmp(&a.name),
//             "day_vol" => b.name.cmp(&a.name),
//             "day_change" => b.name.cmp(&a.name),
//             "name" => b.name.cmp(&a.name),
//             "uppdated_from" => b.name.cmp(&a.name),
//             _ => b.name.cmp(&a.name),
//         }
//     }
// }

static TITLE_KEYS: [&str; 10] = [
    "num",
    "currency",
    "amount",
    "desc",
    "value",
    "price",
    "market_cap",
    "day_vol",
    "day_change",
    "update",
];

// pub fn to_filtered_headers(vec: Vec<String>) -> Vec<String>

// @TODO: if excluded -> continue without adding!
pub fn get_header(conf: &HashMap<String, Value>, visible: Vec<&str>) -> Vec<Cell> {

    let mut map: Vec<Cell> = Vec::new();
    let table_headers = match visible.is_empty() {
        true => TITLE_KEYS.iter(),
        false => visible.iter()
    };

    for def in table_headers {
        let dict: HashMap<String, _> = match conf.get(def.to_owned()) {
            Some(i) => i.to_owned().into_table().unwrap(),
            None => HashMap::new(),
        };

        // custom title
        let title: String = match dict.get("text") {
            Some(val) => val.to_string(),
            None => def.to_string(),
        };

        let align: CellAlignment = match dict.get("align") {
            Some(val) => conf::parse_align(&val.to_string()),
            None => CellAlignment::Left,
        };

        let tint: comfy_table::Color = match dict.get("tint") {
            Some(val) => conf::parse_tint(&val.to_string()),
            None => comfy_table::Color::White,
        };

        // create cell with calculates attributes and add to Vec
        let cell: Cell = Cell::new(title).set_alignment(align).fg(tint);
        map.push(cell)
    }
    map
}

pub fn get_visible_cols(conf: &config::Config) -> Vec<String> {
    // if empty default order is show
    let visible_col = conf.get::<Vec<String>>("table.visible").unwrap_or(vec![]);
    visible_col
}

// accepts [table] field dictionary from config file for styling
// currency of the user for formatting `price` and `value`.
// returns table with headers build in,
pub fn get_skeleton(conf: &config::Config, table_conf: &HashMap<String, Value>, _curr: &String) -> Table {
    // _curr, migh thave used this to dynamically add more price columns prefixed with usd_price , btc_price etc.
    let mut table = Table::new();

    // if empty default order is show
    let visible_col = conf.get::<Vec<&str>>("table.visible").unwrap_or(vec![]);
    // let visible_col = self::get_visible_cols(conf);


    let conf = &table_conf["table"].to_owned().into_table().unwrap();
    let lang_conf = &conf["localize"].to_owned().into_table().unwrap();
    let header_conf = lang_conf["header"].to_owned().into_table().unwrap();

    // let visible_cols: Vec<&str> = conf["visible"].to_owned().try_into().unwrap_or(vec![""]);
    // println!("{:?} {:?}", visible_col, visible_cols);
    let header: Vec<Cell> = get_header(&header_conf, visible_col);
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(header);
    table
}

pub fn sort_rows(conf_two: &config::Config, conf: &HashMap<String, Value>, mut vec: Vec<RowData>) -> Vec<RowData> {

    // let dict: HashMap<String, Value> = conf["table"].to_owned().try_into().unwrap();
    // let sort_dict: HashMap<String, String> = dict["sort"].to_owned().try_into().unwrap();
    // let sort_key: String = match sort_dict.get("key") {
    //     Some(i) => i.to_string(),
    //     None => "num".to_owned(),
    // };

    let sort_key = conf.get::<String>(&"table.sort.key".to_owned());
    let sort_key = match sort_key {
        Some(i) => i.to_string(),
        _ => "num".to_owned()
    };
    //.unwrap_or("num".to_string()).to_string();


    let sort_inverse = conf_two.get::<String>(&"table.sort.inverse".to_owned()).unwrap_or("false".to_string());
    let sort_inverse = match &sort_inverse[..1] {
        // Some(i) => true,
        "t" => true,
        "y" => true,
        _ => false
    };

    // let sort_inverse: bool = match sort_dict.get("inverse") {
    //     Some(i) => match i
    //         .to_string()
    //         .chars()
    //         .next()
    //         .unwrap()
    //         .to_string()
    //         .to_lowercase()
    //         .as_str()
    //     {
    //         "t" => true,
    //         _ => false,
    //     },
    //     None => false,
    // };

    // let sort_inverse: bool = match sort_dict.get("inverse") {
    //     Some(i) => match i
    //         .to_string()
    //         .chars()
    //         .next()
    //         .unwrap()
    //         .to_string()
    //         .to_lowercase()
    //         .as_str()
    //     {
    //         "t" => true,
    //         _ => false,
    //     },
    //     None => false,
    // };

    let index_of_key: usize = TITLE_KEYS.iter().position(|&r| r == sort_key).unwrap_or(0);
    vec.sort_by(|a, b| {
        // b.va
        // println!("{}", TITLE_KEYS[index_of_key]);
        match TITLE_KEYS[index_of_key] {
            // "num" => b.name.cmp(&a.name),
            "currency" => b.currency.cmp(&a.currency),
            "quantity" => ((b.amount * 100.0) as i64).cmp(&((a.amount * 100.0) as i64)),
            "desc" => b.desc.cmp(&a.desc),
            "price" => ((a.price * 100.0) as i64).cmp(&((b.price * 100.0) as i64)),
            "value" => ((a.value * 100.0) as i64).cmp(&((b.value * 100.0) as i64)),
            "day_vol" => ((b.day_vol * 100.0) as i64).cmp(&((a.day_vol * 100.0) as i64)),
            "day_change" => ((b.day_change * 100.0) as i64).cmp(&((a.day_change * 100.0) as i64)),
            "name" => b.currency.cmp(&a.currency),
            "updated_at" => b.updated_at.cmp(&a.updated_at),
            _ => b.currency.cmp(&a.currency),
        }
        // b.get(TITLE_KEYS[index_of_key])
        //     .cmp(&a.get(TITLE_KEYS[index_of_key]))
        // (b as i64).cmp(&(a.amount as i64))
    });

    // it prints out in reverse already, so if they don't want it reverse we do 😱
    if !sort_inverse {
        vec.reverse();
    }

    vec
}

pub fn get_table_row(
    conf_two: &config::Config,
    conf: &HashMap<String, Value>,
    rank: usize,
    vec: &RowData,
    // curr: rusty_money::iso::Currency,
) -> Vec<Cell> {
    let mut map: Vec<Cell> = Vec::new();

    let dict: HashMap<String, Value> = conf["table"].to_owned().try_into().unwrap();
    let local_dict: HashMap<String, Value> = dict["localize"].to_owned().try_into().unwrap();
    let head_dict: HashMap<String, Value> = local_dict["header"].to_owned().try_into().unwrap();

    // rows that are visible in the output
    let visible_cols: Vec<String> = dict["visible"].to_owned().try_into().unwrap_or(vec![]);
    // let visible_cols = self::get_visible_cols(conf_two);
    // let visible_cols = conf.get::<Vec<String>>("table.visible").unwrap_or(vec![]);


    // global defaults for when no settings
    // let def_tint: comfy_table::Color = comfy_table::Color::White;
    // let def_align: CellAlignment = CellAlignment::Left;
    let asset_tint: String = vec.tint.to_owned();

    if !visible_cols.is_empty() {
        println!("Specified Visisble Rows, in-order: {:?}", visible_cols);

        for (n, item) in visible_cols.iter().enumerate() {

            let tit: String = match visible_cols[n].as_str() {
                "num" => (rank+1).to_string(),
                _ => get_field_by_name(vec, item),
            };
            println!(
                "Only add specific row columns in order {:?} {} {}",
                item, tit, visible_cols[n]
            );

            if visible_cols.contains(&visible_cols[n]) {
                let name_cell =
                    self::get_cell(&head_dict, &tit, tit.to_string(), &asset_tint);
                map.push(name_cell);
            }

        }
    } else {
        let num_title: String = "num".to_owned();
        if visible_cols.contains(&num_title) || visible_cols.is_empty() {
            let num_cell =
                self::get_cell(&head_dict, &num_title, (rank + 1).to_string(), &asset_tint);
            map.push(num_cell);
        }

        let currency_title: String = "currency".to_owned();
        if visible_cols.contains(&currency_title) || visible_cols.is_empty() {
            let name_cell = self::get_cell(
                &head_dict,
                &currency_title,
                vec.currency.to_string(),
                &asset_tint,
            );
            map.push(name_cell);
        }

        // let amount_cell = Cell::new(vec.amount.to_string());
        // map.push(amount_cell);
        let amount_title: String = "amount".to_owned();
        if visible_cols.contains(&amount_title) || visible_cols.is_empty() {
            let amount_cell = self::get_cell(
                &head_dict,
                &amount_title,
                vec.amount.to_string(),
                &asset_tint,
            );
            map.push(amount_cell);
        }

        // let desc_cell = Cell::new(vec.desc.to_string());
        // map.push(desc_cell);
        let desc_title: String = "desc".to_owned();
        if visible_cols.contains(&desc_title) || visible_cols.is_empty() {
            let desc_cell =
                self::get_cell(&head_dict, &desc_title, vec.desc.to_string(), &asset_tint);
            map.push(desc_cell);
        }

        // let value_cell = Cell::new(vec.value.to_string());
        // map.push(value_cell);
        // let def_curr = rusty_money::crypto::BTC;
        let def_curr = rusty_money::iso::USD;
        let value_title: String = "value".to_owned();
        if visible_cols.contains(&value_title) || visible_cols.is_empty() {
            let money_format = currency::to_fiat(
                rust_decimal::Decimal::from_f64(vec.value).unwrap(),
                &def_curr,
            );
            let value_cell = self::get_cell(
                &head_dict,
                &value_title,
                money_format.to_string(),
                &asset_tint,
            );
            map.push(value_cell);
        }

        // let price_cell = Cell::new(vec.price.to_string());
        // map.push(price_cell);
        let price_title: String = "price".to_owned();
        if visible_cols.contains(&price_title) || visible_cols.is_empty() {
            let money_format = currency::to_fiat(
                rust_decimal::Decimal::from_f64(vec.price).unwrap(),
                &def_curr,
            );
            let price_cell = self::get_cell(
                &head_dict,
                &price_title,
                money_format.to_string(),
                &asset_tint,
            );
            map.push(price_cell);
        }

        let market_cap_title: String = "market_cap".to_owned();
        if visible_cols.contains(&market_cap_title) || visible_cols.is_empty() {
            // let market_cap = Cell::new(vec.market_cap.to_string());
            // map.push(market_cap);

            // let round_num = match (vec.market_cap / 1_000_000.0).round() / 100.0 {
            //     0.0..=1000.0 => 0.0,
            //     _ => 0.0
            // };

            // let test_round_num = (vec.market_cap / 1_000_000.0).round() / 100.0;
            // let round_num : String = match test_round_num > 1000.0 {
            //     true => (test_round_num / 10.0 ).to_string() + &"B",
            //     false => test_round_num.to_string() + &"M"
            // };

            let round_num = currency::to_shortnum(vec.market_cap);
            // round_num = match round_num > 1000 as f64 {
            //     true => round_num / 1000 as f64,
            //     false => round_num
            // };
            // let round_num : f64 = match (vec.market_cap / 1_000_000.0) {
            //     _ => 0.0
            // };

            // let round_num = (vec.market_cap / 1_000_000.0).round() / 100.0;
            let market_cap = self::get_cell(
                &head_dict,
                &market_cap_title,
                round_num,
                // round_num.to_string() + &"M",
                &asset_tint,
            );
            map.push(market_cap);
        }

        let day_vol_title: String = "day_vol".to_owned();
        if visible_cols.contains(&day_vol_title) || visible_cols.is_empty() {
            // let day_vol = Cell::new(vec.day_vol.to_string());
            // map.push(day_vol);

            // let test_round_num = (vec.day_vol / 1_000_000.0).round() / 100.0;
            // let round_num : String = match test_round_num > 1000.0 {
            //     true => ((test_round_num / 100.0).round() / 100.0).to_string() + &"B",
            //     false => test_round_num.to_string() + &"M"
            // };
            let round_num = currency::to_shortnum(vec.day_vol);
            // let round_num = (vec.day_vol / 1_000_000.0).round() / 100.0;
            let day_vol = self::get_cell(
                &head_dict,
                &day_vol_title,
                round_num,
                // round_num.to_string() + &"M",
                &asset_tint,
            );
            map.push(day_vol);
        }

        let updated_at_title: String = "updated_at".to_owned();
        if visible_cols.contains(&updated_at_title) || visible_cols.is_empty() {
            let day_change_ugly = vec.day_change.round();
            let day_change_color = match day_change_ugly > 0.0 {
                true => comfy_table::Color::Green,
                false => comfy_table::Color::Red,
            };
            let day_change = Cell::new(day_change_ugly.to_string() + &"%").fg(day_change_color);
            map.push(day_change);

            // let updated_at_rought = vec.updated_at.to_string()
            let ti = chrono::Utc.timestamp(vec.updated_at as i64, 0);
            let last_update_min = Utc::now().signed_duration_since(ti).num_minutes();
            let ht = last_update_min.to_string() + &"m";
            let updated_at_col: comfy_table::Color = match last_update_min {
                0..=2 => comfy_table::Color::Green,
                3..=5 => comfy_table::Color::Yellow,
                _ => comfy_table::Color::Red,
            };
            let updated_at = Cell::new(ht).fg(updated_at_col);
            map.push(updated_at);
        }
    }

    map
}

pub fn get_cell(
    head_dict: &HashMap<String, Value>,
    section: &str,
    data: String,
    tint: &String,
) -> Cell {
    // global defaults for when no settings
    let def_tint: comfy_table::Color = comfy_table::Color::White;
    let def_align: CellAlignment = CellAlignment::Left;

    // println!("get_cell: {:?} {} {} {}", head_dict, section, data, tint);

    let num_head_dict = match head_dict.get(section) {
        Some(i) => {
            let it: HashMap<String, Value> = i.to_owned().try_into().unwrap();
            // println!("{:?}", it);
            it
        }
        None => HashMap::new(),
    };

    let num_dict = match num_head_dict.get("rows") {
        Some(i) => {
            let it: HashMap<String, Value> = i.to_owned().try_into().unwrap();
            // println!("{:?}", it);
            it
        }
        None => HashMap::new(),
    };

    // @TODO : pretty sure if no tint it set for a certain row it crashes D:
    let num_tint = match num_dict.get("tint") {
        Some(i) => {
            let is = i.to_string();
            // println!("{:?} {:?}", is, vec.tint);
            match is.as_str() {
                "tint" => conf::parse_tint(&tint),
                "" => def_tint,
                _ => conf::parse_tint(&is),
            }
        }
        None => def_tint,
    };

    let num_align = match num_dict.get("align") {
        Some(i) => conf::parse_align(&i.to_string()),
        None => def_align,
    };

    let index_cell = Cell::from((data).to_string())
        .set_alignment(num_align)
        .fg(num_tint);
    // map.push(index_cell);
    index_cell
}
