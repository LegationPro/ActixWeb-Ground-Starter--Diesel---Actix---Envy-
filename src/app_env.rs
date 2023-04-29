use envy;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

pub fn init() -> Config {
    dotenv().ok().unwrap();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    }
}
