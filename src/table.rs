use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

#[path = "./config.rs"]
mod conf;
use config::Value;
use std::collections::HashMap;

pub fn get_skeleton(table_conf: &HashMap<String, Value>, curr: &String) -> Table {
    let mut table = Table::new();
    let titles = [
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
    let conf = &table_conf["table"].to_owned().into_table().unwrap();
    let lang_conf = &conf["localize"].to_owned().into_table().unwrap();
    let header_conf = lang_conf["header"].to_owned().into_table().unwrap();
    let mut map = HashMap::new();
    for i in titles.iter() {

        // let defaults: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
        //     .iter()
        //     .cloned()
        //     .collect();

        let title: HashMap<String, _> = match header_conf.get(i.to_owned()) {
            Some(i) => i.to_owned().into_table().unwrap(),
            None => HashMap::new(),
        };

        // title.entry("text".to_owned()).or_insert(i.to_owned());
        // title.entry("align".to_owned()).or_insert("center");


        // let tit = i.to_string();
        // conf.localize.header.num;
        // let this_conf = &header_conf[tit].to_owned().into_table().unwrap();
        // let title_dict = header_conf[&i.to_string()];
        // let title_dict = conf["localize"].into_table().unwrap();
        // let header_dict = title_dict["header"].into_table().unwrap();
        println!("{:?} ", title);
        // let this_dict = header_dict[i].into_table().unwrap();

        map.insert(i, Cell::new(i));
    }

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#")
                .add_attribute(Attribute::Bold)
                .fg(comfy_table::Color::Green),
            Cell::new("currency"),
            Cell::new("quantity"),
            Cell::new("desc"),
            Cell::new(curr.to_owned().to_lowercase() + &"_value"),
            Cell::new(curr.to_owned().to_lowercase() + &"_price"),
            Cell::new("market_cap"),
            Cell::new("24hr_vol"),
            Cell::new("24hr_diff"),
            Cell::new("update"),
            // "Allocation %",
        ]);
    table
}

pub fn get_row(vec: Vec<String>) -> () {
    let mut row = Vec::new();
    for i in vec.iter() {
        row.push(Cell::new(i));
    }
}
