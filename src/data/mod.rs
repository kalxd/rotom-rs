use std::sync::Arc;

use sqlx::PgPool;

pub mod config;
pub mod error;

#[derive(Debug, Clone)]
pub struct AppState {
	salt: Arc<String>,
	db: PgPool,
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
