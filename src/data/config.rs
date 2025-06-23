use serde::Deserialize;

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
	todo!()
}
