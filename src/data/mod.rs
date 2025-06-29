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
	pub id: i32,
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
			.map_err(error::Error::not_auth)?;

		let uuid = ty::Uuid::try_from(token).map_err(error::Error::not_auth)?;

		let state = req
			.app_state::<AppState>()
			.ok_or(error::Error::internal("依赖未成功注入！"))?;

		let user = sqlx::query_as!(
			User,
			r#"
select u.编号 as id, u.用户名 as username from 用户会话 as s
inner join 用户 as u on u.编号 = s.用户编号
where s.令牌 = $1
"#,
			uuid as ty::Uuid
		)
		.fetch_optional(&state.db)
		.await?;

		user.ok_or(error::Error::not_auth("用户不存在！"))
	}
}
