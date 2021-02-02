use config::{Config, Environment, File, Value};

pub fn get_config() -> Config {
    let f = File::with_name("coinprs");
    let mut settings = Config::default();
    settings
        .merge(f)
        .unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(Environment::with_prefix("APP"))
        .unwrap();
    settings
}

use comfy_table::*;
pub fn parse_align(arg1: &Value) -> comfy_table::CellAlignment {
    let first_char = arg1
        .to_string()
        .chars()
        .next()
        .unwrap()
        .to_string()
        .to_lowercase();
    match first_char.as_str() {
        "l" => CellAlignment::Left,
        "c" => CellAlignment::Center,
        "r" => CellAlignment::Right,
        _ => CellAlignment::Left,
    }
}

use comfy_table::Color::Rgb;
pub fn parse_tint(arg1: &Value) -> comfy_table::Color {
    let first_char = arg1.to_string().chars().next().unwrap().to_string();
    match first_char == "#" {
        true => {
            let row_tint = tint::Color::from(arg1.to_string());
            Rgb {
                r: (row_tint.red * 255.0) as u8,
                g: (row_tint.green * 255.0) as u8,
                b: (row_tint.blue * 255.0) as u8,
            }
        }
        false => comfy_table::Color::White,
    }
}
