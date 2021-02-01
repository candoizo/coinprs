use config::{Config, Environment, File};

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
