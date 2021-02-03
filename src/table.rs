use comfy_table::presets::UTF8_FULL;
use comfy_table::Color::Rgb;
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

        let align: CellAlignment = match dict.get("align") {
            Some(val) => conf::parse_align(val),
            None => CellAlignment::Left,
        };

        let tint: comfy_table::Color = match dict.get("tint") {
            Some(val) => conf::parse_tint(val),
            None => comfy_table::Color::White,
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
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(header);
    table
}

pub fn get_row(
    conf: &HashMap<String, Value>,
    rank: usize,
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

    let index_cell = Cell::from(rank.to_string());
    map.push(index_cell);

    for (i, item) in vec.iter().enumerate() {
        // println!("{:?} {:?}", head_dict, item);
        let row_dict: HashMap<String, Value> = match head_dict.get(TITLE_KEYS[i]) {
            Some(i) => {
                let m_dict: HashMap<String, Value> = i.to_owned().try_into().unwrap();
                // println!("{:?}", m_dict);
                match m_dict.get("rows") {
                    Some(i) => i.to_owned().try_into().unwrap(),
                    None => HashMap::new(),
                }
            }
            None => HashMap::new(),
        };

        let mut row_tint: comfy_table::Color = match row_dict.get("tint") {
            Some(val) => match val.to_string() == "tint" {
                true => Rgb {
                    r: (row_tint.red * 255.0) as u8,
                    g: (row_tint.green * 255.0) as u8,
                    b: (row_tint.blue * 255.0) as u8,
                },
                false => conf::parse_tint(val),
            },
            None => def_tint,
        };
        // @TODO why was this not suitable?
        // let tint: comfy_table::Color = match dict.get("tint") {
        //     Some(val) => conf::parse_tint(val),
        //     None => comfy_table::Color::White,
        // };

        let row_align: comfy_table::CellAlignment = match row_dict.get("align") {
            Some(val) => conf::parse_align(val),
            None => def_align,
        };

        // perhaps do some global that applies to
        // $ sign money
        // time (m)
        // % percentage
        // abc string
        // 0.0 qty

        if TITLE_KEYS[i] == "update" {
            let num: usize = item
                .to_string()
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
            row_tint = match num {
                0..=2 => comfy_table::Color::Green,
                3..=5 => comfy_table::Color::Yellow,
                _ => comfy_table::Color::Red,
            }
        } else if TITLE_KEYS[i] == "24hr_diff" {
            let f_char: String = item.to_string().chars().next().unwrap().to_string();
            row_tint = match f_char.as_str() {
                "-" => comfy_table::Color::Red,
                _ => comfy_table::Color::Green,
            }
        }

        let cell = Cell::new(item.to_string())
            .set_alignment(row_align)
            .fg(row_tint);
        map.push(cell);
    }

    map
}

pub fn sort_rows(conf: &HashMap<String, Value>, mut vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let dict: HashMap<String, Value> = conf["table"].to_owned().try_into().unwrap();

    // for or_insert requires
    let sort_dict: HashMap<String, String> = dict["sort"].to_owned().try_into().unwrap();
    // let vecs = Vec::new();
    let sort_key: String = match sort_dict.get("key") {
        Some(i) => i.to_string(),
        None => "num".to_owned(),
    };
    // requires a mutable vector, which idk
    // let sort_keyy = sort_dict.entry("key".to_owned()).or_insert("num".to_owned());
    // let def_key = "num".to_string();
    // let sort_keyy = sort_dict.get("key").unwrap_or(&def_key);

    // let def_sort_inverse = "false".to_owned();
    // let sort_inverse = sort_dict.get("inverse").unwrap_or(&def_sort_inverse);
    let sort_inverse: bool = match sort_dict.get("inverse") {
        Some(i) => match i
            .to_string()
            .chars()
            .next()
            .unwrap()
            .to_string()
            .to_lowercase()
            .as_str()
        {
            "t" => true,
            _ => false,
        },
        None => false,
    };

    println!("{0} {1}", sort_key, sort_inverse);
    // let sort_key : String = sort_dict.get("key").to_string().unwrap();
    // let key_exi = Some(sort_key);
    // let key = key_exi.unwrap_or_else(|| {
    //     String::from("Abc")
    //     });
    // let sort_key : String = sort_dict.get("key").to_owned().unwrap_or("abc").to_string();
    // let val : String = sort_dict.get("inverse").to_string().unwrap();
    // println!("{:?}", val);
    // let sort_inverse : String = sort_dict.get("inverse").unwrap().try_into().unwrap();
    // let sort_inverse : bool = sort_dict.get("inverse").unwrap_or(&Value{"false".to_string}).try_into().unwrap();
    // let sort_inverse : bool = match sort_dict.get("inverse") {
    //     Some(i) => i.into_bool().unwrap(),
    //     None => "num".to_owned()
    // };

    let index_of_key : usize = TITLE_KEYS.iter().position(|&r| r == sort_key).unwrap_or(0);
    println!("{}", index_of_key);
    // let index_of_sort_key = TITLE_KEYS.index

    // probably need to check if the first value is a number or string before sorting
    // if reverse then * -1 to flip the val

    // @TODO!!!!!: SEE THE +1 after index of key, thats because i shouldnt add the row counts early like I did
    vec.sort_by(|a, b| b[index_of_key+1].cmp(&a[index_of_key+1]));
    // vecs
    vec
}
