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
                .about("Sets a custom config file")
                .takes_value(true),
        )
        // .arg(
        //     Arg::new("INPUT")
        //         .about("Sets the input file to use")
        //         .required(true)
        //         .index(1),
        // )
        // .arg(
        //     Arg::new("v")
        //         .short('v')
        //         .multiple(true)
        //         .about("Sets the level of verbosity"),
        // )
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

        let conf_path = match matches.value_of("config") {
            Some(i) => i,
            None => "coinprs"
        };
        println!("{}", conf_path);
        // if (matches.is_present("config")) {
        //     println!("CUSTOM CONFIGURATION?! {:?}", matches.value_of("config"));
        // } else {
        //     println!("Using default config!");
        // }




    matches
}

use config::{Config, Environment, File, Value};

pub fn get_config() -> Config {

    let matches = self::get_opts();
    let conf_path = match matches.value_of("config") {
        Some(i) => i,
        None => "coinprs"
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
    // let first_char = arg1.to_string().chars().next().unwrap().to_string();
    // match first_char == "#" {
    //     true => {
    //         let row_tint = tint::Color::from(arg1.to_string());
    //         Rgb {
    //             r: (row_tint.red * 255.0) as u8,
    //             g: (row_tint.green * 255.0) as u8,
    //             b: (row_tint.blue * 255.0) as u8,
    //         }
    //     }
    //     false => comfy_table::Color::White,
    // }
    let row_tint = tint::Color::from(arg1.to_string());
    Rgb {
        r: (row_tint.red * 255.0) as u8,
        g: (row_tint.green * 255.0) as u8,
        b: (row_tint.blue * 255.0) as u8,
    }
}
