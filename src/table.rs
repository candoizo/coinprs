use comfy_table::presets::UTF8_FULL;
use comfy_table::*;


pub fn get_skeleton(curr: &String) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#")
                .add_attribute(Attribute::Bold)
                .fg(comfy_table::Color::Green),
            Cell::new("currency"),
            Cell::new("quantity"),
            Cell::new(curr.to_owned().to_lowercase() + &"_value"),
            Cell::new(curr.to_owned().to_lowercase() + &"_price"),
            Cell::new("market_cap"),
            Cell::new("24hr_vol"),
            Cell::new("24hr_diff"),
            Cell::new("data_update"),
            // "Allocation %",
        ]);
    table
}
