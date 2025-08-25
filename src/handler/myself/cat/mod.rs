use ntex::web::{
	DefaultError, Scope, get, post, scope,
	types::{Json, State},
};
use serde::{Deserialize, Serialize};

use crate::data::{AppState, User, error::Result, ty::UpdateBody};

#[derive(Debug, sqlx::FromRow, Serialize)]
struct CatWithCount {
	id: Option<i32>,
	name: String,
	count: i64,
}

#[get("/list")]
async fn get_all_cat(user: User, state: State<AppState>) -> Result<Json<Vec<CatWithCount>>> {
	let cats = sqlx::query_file_as!(CatWithCount, "sql/cat/get_all_cat.sql", &user.id)
		.fetch_all(&state.db)
		.await?;

	Ok(Json(cats))
}

#[derive(Debug, Deserialize)]
struct CreateCatBody {
	name: String,
}

#[post("/create")]
async fn create_cat(
	body: Json<CreateCatBody>,
	user: User,
	state: State<AppState>,
) -> Result<Json<CatWithCount>> {
	let cat = sqlx::query_as!(
		CatWithCount,
		r#"
insert into 分类
(用户编号, 名称)
values ($1, $2)
returning 编号 as id, 名称 as name, 0 as "count!"
"#,
		user.id,
		body.name
	)
	.fetch_one(&state.db)
	.await?;

	Ok(Json(cat))
}

#[derive(Debug, Deserialize)]
struct UpdateCatBody {
	name: String,
}

#[post("/update")]
async fn update_cat(
	body: Json<UpdateBody<UpdateCatBody>>,
	user: User,
	state: State<AppState>,
) -> Result<Json<Option<CatWithCount>>> {
	let cat = sqlx::query_file_as!(
		CatWithCount,
		"sql/cat/update_cat.sql",
		user.id,
		body.id,
		body.data.name
	)
	.fetch_optional(&state.db)
	.await?;

	Ok(Json(cat))
}

pub fn api() -> Scope<DefaultError> {
	scope("/cat")
		.service(get_all_cat)
		.service(create_cat)
		.service(update_cat)
}
