#![allow(dead_code)]
use clap::{App, Arg, ArgMatches};
pub fn get_opts() -> ArgMatches {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Set path to a custom config file")
                .takes_value(true),
        )
        .subcommand(
            App::new("show")
                .about("print list of supported assets")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )
        .subcommand(
            App::new("price")
                .about("print asset price")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely"),
                ),
        )
        .subcommand(
            App::new("report")
                .about("print portfolio table")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('r')
                        .about("print debug information verbosely"),
                ),
        )
        .get_matches();

    matches
}

use config::{Config, Environment, File};
pub fn get_config() -> Config {
    let matches = self::get_opts();
    let conf_path = match matches.value_of("config") {
        Some(i) => i,
        None => "coinprs",
    };

    let f = File::with_name(conf_path);
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

use comfy_table::CellAlignment;
pub fn parse_align(arg1: &str) -> comfy_table::CellAlignment {
    match &arg1[..1] {
        "l" => CellAlignment::Left,
        "c" => CellAlignment::Center,
        "r" => CellAlignment::Right,
        _ => CellAlignment::Left,
    }
    // let first_char = arg1
    //     .to_string()
    //     .chars()
    //     .next()
    //     .unwrap()
    //     .to_string()
    //     .to_lowercase();
    // match first_char.as_str() {
    //     "l" => CellAlignment::Left,
    //     "c" => CellAlignment::Center,
    //     "r" => CellAlignment::Right,
    //     _ => CellAlignment::Left,
    // }
}

use comfy_table::Color::Rgb;
pub fn parse_tint(arg1: &String) -> comfy_table::Color {
    // println!("Parsing tint for: {}", arg1);

    // if first char != "#",
    // check if tint::Color::names().contains(thing);

    // let tint_test : tint::Color = tint::Color::from(arg1.to_owned());
    let try_tint = std::panic::catch_unwind(|| tint::Color::from(arg1.to_owned()));
    let row_tint = try_tint.unwrap_or(tint::Color::from("#FFFFFF"));
    // let row_tint : tint::Color =  test_test(&arg1.to_owned()).unwrap();//test_test(&arg1.to_owned()).unwrap_or(tint::Color::from("#FFFFFF"));
    // println!("Parsing tint again for: {:?}", row_tint);
    // let row_tint = tint::Color::from(arg1.to_string());
    Rgb {
        r: (row_tint.red * 255.0) as u8,
        g: (row_tint.green * 255.0) as u8,
        b: (row_tint.blue * 255.0) as u8,
    }
}
