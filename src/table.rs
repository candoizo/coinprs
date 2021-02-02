use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

#[path = "./config.rs"]
mod conf;
use config::Value;
use std::collections::HashMap;

const TITLE_KEYS: [&str; 10] = [
    "num",
    "currency",
    "quantity",
    "desc",
    "value",
    "price",
    "market_cap",
    "24hr_vol",
    "24hr_diff",
    "update",
];

pub fn get_header(conf: &HashMap<String, Value>) -> Vec<Cell> {
    // let mut map = HashMap::new();
    let mut map: Vec<Cell> = Vec::new();
    for def in TITLE_KEYS.iter() {
        let dict: HashMap<String, _> = match conf.get(def.to_owned()) {
            Some(i) => i.to_owned().into_table().unwrap(),
            None => HashMap::new(),
        };

        // custom title
        let title: String = match dict.get("text") {
            Some(val) => val.to_string(),
            None => def.to_string(),
        };

        let default_align: CellAlignment = CellAlignment::Left;
        let align: CellAlignment = match dict.get("align") {
            Some(val) => {
                let first_char = val
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .to_lowercase();
                if first_char == "l" {
                    CellAlignment::Left
                } else if first_char == "c" {
                    CellAlignment::Center
                } else if first_char == "r" {
                    CellAlignment::Right
                } else {
                    default_align
                }
            }
            None => default_align,
        };

        let default_tint: comfy_table::Color = comfy_table::Color::White;
        let tint: comfy_table::Color = match dict.get("tint") {
            Some(val) => {
                let first_char = val.to_string().chars().next().unwrap().to_string();
                if first_char == "#" {
                    // hex code with #
                    comfy_table::Color::White
                } else {
                    default_tint
                }
            }
            None => default_tint,
        };

        // create cell with calculates attributes and add to Vec
        let cell: Cell = Cell::new(title).set_alignment(align).fg(tint);
        map.push(cell)
    }
    map
}

// accepts [table] field dictionary from config file for styling
// currency of the user for formatting `price` and `value`.
// returns table with headers build in,
pub fn get_skeleton(table_conf: &HashMap<String, Value>, curr: &String) -> Table {
    let mut table = Table::new();
    let conf = &table_conf["table"].to_owned().into_table().unwrap();
    let lang_conf = &conf["localize"].to_owned().into_table().unwrap();
    let header_conf = lang_conf["header"].to_owned().into_table().unwrap();
    let header: Vec<Cell> = get_header(&header_conf);
    // println!("{:?}", header);

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(header);
    table
}

pub fn get_row(conf: &HashMap<String, Value>,vec: Vec<String>) -> Vec<Cell> {
    let mut map: Vec<Cell> = Vec::new();

    for i in vec.iter() {
        map.push(Cell::new(i));
    }

    map
}
