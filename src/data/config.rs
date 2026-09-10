use ralts::config;
use ralts::error::Result;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgConnectOptions};

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
	pub salt: String,
	pub database: ConfigDb,
}

impl Config {
	pub fn load_config() -> Result<Self> {
		config::load_config()
	}

	pub async fn make_db_connection(&self) -> Result<PgPool> {
		let option = PgConnectOptions::new()
			.host(&self.database.host)
			.port(self.database.port)
			.username(&self.database.user)
			.password(&self.database.password)
			.database(&self.database.db);

		let pool = PgPool::connect_with(option).await?;
		Ok(pool)
	}
}
