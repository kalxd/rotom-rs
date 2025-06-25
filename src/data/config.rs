use config::{Config as ConfigLib, File};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgConnectOptions};
use std::fs;

use super::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct ConfigDb {
	db: String,
	user: String,
	host: String,
	port: u16,
	password: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	salt: String,
	database: ConfigDb,
}

pub fn load_config() -> Result<Config> {
	const DEFAULT_CONFIG: &str = "config/default.toml";
	const CUSTOM_CONFIG: &str = "config/config.toml";

	let config = ConfigLib::builder().add_source(File::with_name(DEFAULT_CONFIG));
	let config = if fs::exists(CUSTOM_CONFIG).ok() == Some(true) {
		config.add_source(File::with_name(CUSTOM_CONFIG))
	} else {
		config
	};

	config
		.build()
		.map_err(Error::internal)?
		.try_deserialize()
		.map_err(Error::internal)
}

impl Config {
	pub async fn make_db_connection(&self) -> Result<PgPool> {
		let option = PgConnectOptions::new()
			.host(&self.database.host)
			.port(self.database.port)
			.username(&self.database.user)
			.password(&self.database.password)
			.database(&self.database.db);

		PgPool::connect_with(option).await.map_err(Error::internal)
	}
}
