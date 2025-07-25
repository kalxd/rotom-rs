use ntex::web::{
	DefaultError, Scope, post, scope,
	types::{Json, State},
};
use serde::{Deserialize, Serialize};

use crate::data::{
	AppState, User,
	error::{Error, Result},
	ty::{SaltPassword, Uuid},
};

mod cat;
mod emoji;
mod file;
mod user;

#[derive(Debug, Serialize)]
struct SessionUser {
	token: Uuid,
	user: User,
}

#[derive(Debug, Deserialize)]
struct LoginBody {
	username: String,
	password: String,
}

#[post("/login")]
async fn login(body: Json<LoginBody>, state: State<AppState>) -> Result<Json<SessionUser>> {
	let salt = SaltPassword::new(&body.password, &state.salt);

	let user = sqlx::query_as!(
		User,
		r#"
select
编号 as id, 用户名 as username
from 用户
where 用户名 = $1 and 密码 = md5($2)
"#,
		&body.username,
		&salt as &SaltPassword
	)
	.fetch_optional(&state.db)
	.await?
	.ok_or(Error::not_auth("用户名或密码不正确！"))?;

	let token = sqlx::query_scalar!(
		r#"
insert into
用户会话 (用户编号)
values ($1)
returning 令牌 as "token!: Uuid"
"#,
		user.id
	)
	.fetch_one(&state.db)
	.await?;

	Ok(Json(SessionUser { token, user }))
}

pub fn api() -> Scope<DefaultError> {
	scope("/")
		.service(login)
		.service(user::api())
		.service(cat::api())
		.service(file::api())
		.service(emoji::api())
}
