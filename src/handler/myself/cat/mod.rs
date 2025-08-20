use ntex::web::{
	DefaultError, Scope, get, post, scope,
	types::{Json, State},
};
use serde::{Deserialize, Serialize};

use crate::data::{AppState, User, error::Result, ty::UpdateBody};
use crate::helper::cat;

#[derive(Debug, sqlx::FromRow, Serialize)]
struct CatWithCount {
	id: i32,
	name: String,
	count: i64,
}

#[get("/list")]
async fn get_all_cat(user: User, state: State<AppState>) -> Result<Json<Vec<CatWithCount>>> {
	let cats = sqlx::query_as!(
		CatWithCount,
		r#"
select
cat.编号 as id, cat.名称 as name, t.count as "count!"
from
分类 as cat,
lateral (select count(编号) as count from 表情 where 表情.分类编号 = cat.编号) as t
where 用户编号 = $1
order by id desc
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
async fn create_cat(
	body: Json<CreateCatBody>,
	user: User,
	state: State<AppState>,
) -> Result<Json<cat::Cat>> {
	let cat = sqlx::query_as!(
		cat::Cat,
		r#"
insert into 分类
(用户编号, 名称)
values ($1, $2)
returning 编号 as id, 名称 as name
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
) -> Result<Json<Option<cat::Cat>>> {
	let cat = sqlx::query_as!(
		cat::Cat,
		r#"
update 分类
set 名称 = $1
where 编号 = $2 and 用户编号 = $3
returning 编号 as id, 名称 as name
"#,
		body.data.name,
		body.id,
		user.id
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
