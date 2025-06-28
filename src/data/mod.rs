use std::sync::Arc;

use ntex::web::{ErrorRenderer, FromRequest};
use serde::Serialize;
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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
	#[sqlx(rename = "编号")]
	pub id: i32,
	#[sqlx(rename = "用户名")]
	pub username: String,
}

impl<E: ErrorRenderer> FromRequest<E> for User {
	type Error = error::Error;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		_: &mut ntex::http::Payload,
	) -> error::Result<Self> {
		let token = req
			.headers()
			.get("XGToken")
			.ok_or(error::Error::not_auth("未填写令牌！"))?
			.to_str()
			.map_err(|e| error::Error::not_auth(e))?;

		let state = req
			.app_state::<AppState>()
			.ok_or(error::Error::internal("依赖未成功注入！"))?;

		todo!()
	}
}
