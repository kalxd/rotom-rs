use std::sync::Arc;

use ntex::web::{ErrorRenderer, FromRequest};
use ralts::error::{AppError, Result};
use serde::Serialize;
use sqlx::PgPool;

pub mod config;
pub mod file;
pub mod ty;

#[derive(Debug, Clone, drv::Database)]
pub struct AppState {
	pub salt: Arc<String>,
	#[database]
	pub db: PgPool,
}

impl AppState {
	pub async fn from_config(cfg: &config::Config) -> Result<Self> {
		let pool = cfg.make_db_connection().await?;

		Ok(Self {
			salt: Arc::new(cfg.salt.clone()),
			db: pool,
		})
	}
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
	pub id: i32,
	pub username: String,
}

impl<E: ErrorRenderer> FromRequest<E> for User {
	type Error = AppError;

	async fn from_request(
		req: &ntex::web::HttpRequest,
		_: &mut ntex::http::Payload,
	) -> Result<Self> {
		let token = req
			.headers()
			.get("XGToken")
			.ok_or(AppError::no_auth("未填写令牌！"))?
			.to_str()
			.map_err(AppError::no_auth)?;

		let uuid = ty::Uuid::try_from(token).map_err(AppError::no_auth)?;

		let state = req
			.app_state::<AppState>()
			.ok_or(AppError::internal("依赖未成功注入！"))?;

		let user = sqlx::query_as!(
			User,
			r#"
select u.编号 as id, u.用户名 as username from 用户会话 as s
inner join 用户 as u on u.编号 = s.用户编号
where s.令牌 = $1
"#,
			uuid as ty::Uuid
		)
		.fetch_optional(state)
		.await?;

		user.ok_or(AppError::no_auth("用户不存在！"))
	}
}
