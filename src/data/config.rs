use config::{Config as ConfigLib, File};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ConfigDb {
	db: String,
	user: String,
	host: String,
	port: String,
	password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	salt: String,
	database: ConfigDb,
}

pub fn load_config() -> Config {
	const DEFAULT_CONFIG: &str = "config/default.toml";
	const CUSTOM_CONFIG: &str = "config/config.toml";

	let config = ConfigLib::builder().add_source(File::with_name(DEFAULT_CONFIG));
	let config = if fs::exists(CUSTOM_CONFIG).ok() == Some(true) {
		config.add_source(File::with_name(CUSTOM_CONFIG))
	} else {
		config
	};

	config.build().unwrap().try_deserialize().unwrap()
}
