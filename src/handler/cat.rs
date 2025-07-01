use ntex::web::{
	DefaultError, Scope, get, post, scope,
	types::{Json, State},
};
use serde::{Deserialize, Serialize};

use crate::data::{AppState, User, error::Result};

#[derive(Debug, sqlx::FromRow, Serialize)]
struct Cat {
	id: i32,
	name: String,
}

#[get("/list")]
async fn get_all_cat(user: User, state: State<AppState>) -> Result<Json<Vec<Cat>>> {
	let cats = sqlx::query_as!(
		Cat,
		r#"
select
cat.编号 as id, cat.名称 as name
from 分类 as cat
where 用户编号 = $1
"#,
		user.id
	)
	.fetch_all(&state.db)
	.await?;

	Ok(Json(cats))
}

#[derive(Debug, Deserialize)]
struct CreateCatBody {
	name: String,
}

#[post("/create")]
async fn create_cat(body: Json<CreateCatBody>) -> Result<Json<()>> {
	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/self/cat").service(get_all_cat).service(create_cat)
}
