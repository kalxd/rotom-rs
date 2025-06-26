use std::sync::Arc;

use sqlx::PgPool;

pub mod config;
pub mod error;
pub mod ty;

#[derive(Debug, Clone)]
pub struct AppState {
	pub salt: Arc<String>,
	pub db: PgPool,
}

impl AppState {
	pub async fn from_config(cfg: &config::Config) -> error::Result<Self> {
		let pool = cfg.make_db_connection().await?;

		Ok(Self {
			salt: Arc::new(cfg.salt.clone()),
			db: pool,
		})
	}
}
