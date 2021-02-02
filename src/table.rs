use comfy_table::presets::UTF8_FULL;
use comfy_table::Color::Rgb;
use comfy_table::*;
use tint::Color;
// use tint::*;

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
            Some(val) => conf::parse_align(val),
            None => default_align,
        };

        let default_tint: comfy_table::Color = comfy_table::Color::White;
        let tint: comfy_table::Color = match dict.get("tint") {
            Some(val) => {
                let first_char = val.to_string().chars().next().unwrap().to_string();
                if first_char == "#" {
                    // hex code with #
                    let row_tint = tint::Color::from(val.to_string());
                    Rgb {
                        r: (row_tint.red * 255.0) as u8,
                        g: (row_tint.green * 255.0) as u8,
                        b: (row_tint.blue * 255.0) as u8,
                    }
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

// pub fn tint_change()

pub fn get_row(
    conf: &HashMap<String, Value>,
    vec: Vec<String>,
    row_tint: tint::Color,
) -> Vec<Cell> {
    let mut map: Vec<Cell> = Vec::new();

    let dict: HashMap<String, Value> = conf["table"].to_owned().try_into().unwrap();
    let local_dict: HashMap<String, Value> = dict["localize"].to_owned().try_into().unwrap();
    let head_dict: HashMap<String, Value> = local_dict["header"].to_owned().try_into().unwrap();

    // global defaults for when no settings
    let def_tint: comfy_table::Color = comfy_table::Color::White;
    let def_align: CellAlignment = CellAlignment::Left;

    for (i, item) in vec.iter().enumerate() {
        println!("{:?} {:?}", head_dict, item);
        let row_dict : HashMap<String, Value> = match head_dict.get(TITLE_KEYS[i]) {
            Some(i) => {
                let m_dict: HashMap<String, Value> = i.to_owned().try_into().unwrap();
                println!("{:?}", m_dict);
                match m_dict.get("rows") {
                    Some(i) => {
                        i.to_owned().try_into().unwrap()
                        },
                    None => {
                        HashMap::new()
                    }
                }
                },
            None => HashMap::new()
        };

        let row_tint : comfy_table::Color = match row_dict.get("tint") {
            Some(val) => {
                let first_char = val.to_string().chars().next().unwrap().to_string();
                if first_char == "#" {
                    // hex code with #
                    let row_tint = tint::Color::from(val.to_string());
                    Rgb {
                        r: (row_tint.red * 255.0) as u8,
                        g: (row_tint.green * 255.0) as u8,
                        b: (row_tint.blue * 255.0) as u8,
                    }
                } else {
                    def_tint
                }
                },
            None => def_tint
        };

        let row_align : comfy_table::CellAlignment = match row_dict.get("align") {
            Some(val) => conf::parse_align(val),
            None => def_align
        };

        // // let row_dict: HashMap<String, Value> = header_dict.to_owned()[&i.to_string()];
        // let mut tint: comfy_table::Color = comfy_table::Color::White;
        // if TITLE_KEYS[i] == "update" {
        //     // set tint based on time since update
        //     let num: usize = item
        //         .to_string()
        //         .chars()
        //         .next()
        //         .unwrap()
        //         .to_string()
        //         .parse()
        //         .unwrap();
        //     tint = match num {
        //         0..=2 => comfy_table::Color::Green,
        //         3..=5 => comfy_table::Color::Yellow,
        //         _ => comfy_table::Color::Red,
        //     }
        // } else if TITLE_KEYS[i] == "24hr_diff" {
        //     let ii = item.to_string();
        //     let mut chars = ii.chars();
        //     let f_char = chars.next().unwrap().to_string();
        //     let l_char = chars.last().unwrap().to_string();
        //     if l_char == "%" {
        //         tint = match f_char == "-" {
        //             true => comfy_table::Color::Red,
        //             false => comfy_table::Color::Green,
        //         };
        //     }
        // } else if TITLE_KEYS[i] == "currency" {
        //     tint = Rgb {
        //         r: (row_tint.red * 255.0) as u8,
        //         g: (row_tint.green * 255.0) as u8,
        //         b: (row_tint.blue * 255.0) as u8,
        //     };
        // } else if TITLE_KEYS[i] == "desc" {
        // } else if TITLE_KEYS[i] == "value" {
        //     tint = comfy_table::Color::Cyan;
        // } else if TITLE_KEYS[i] == "price" {
        //     tint = comfy_table::Color::Cyan;
        // } else if TITLE_KEYS[i] == "num" {
        //     tint = comfy_table::Color::Grey;
        // }

        // let cell = Cell::new(item.to_string());//.fg(tint);
        let cell = Cell::new(item.to_string()).set_alignment(row_align).fg(row_tint);
        map.push(cell);
    }

    map
}
