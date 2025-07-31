use ntex::web::{DefaultError, Scope, post, scope, types::Json};
use serde::{Deserialize, Serialize};

use crate::data::{
	User,
	error::{Error, Result},
	ty::UpdateBody,
};
use crate::helper;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBody {
	file_sha: String,
	cat_id: Option<i32>,
	desc: Option<String>,
}

#[derive(Debug, Clone, drv::State, drv::Database)]
struct EmojiState {
	#[database]
	file: helper::file::FileState,
}

impl EmojiState {
	async fn get_user_cat(&self, user_id: &i32, cat_id: &i32) -> Result<Option<helper::cat::Cat>> {
		let cat = sqlx::query_as!(
			helper::cat::Cat,
			r#"
select 编号 as id, 名称 as name
from 分类
where 编号 = $1 and 用户编号 = $2
"#,
			cat_id,
			user_id
		)
		.fetch_optional(self)
		.await?;

		Ok(cat)
	}

	async fn check_user_cat(&self, user_id: &i32, cat_id: &i32) -> Result<helper::cat::Cat> {
		self.get_user_cat(user_id, cat_id)
			.await?
			.ok_or(Error::illegal("所选分类不存在！"))
	}
}

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct Emoji {
	id: i32,
	cat_id: Option<i32>,
	file_sha: String,
	desc: Option<String>,
}

#[post("/create")]
async fn create_emoji(
	user: User,
	body: Json<CreateBody>,
	state: EmojiState,
) -> Result<Json<Emoji>> {
	state.file.check_file_by_sha(&body.file_sha).await?;

	if let Some(ref cat_id) = body.cat_id {
		state.check_user_cat(&user.id, cat_id).await?;
	}

	let emoji = sqlx::query_as!(
		Emoji,
		r#"
insert into 表情
(用户编号, 分类编号, 文件特征, 描述)
values ($1, $2, $3, $4)
returning 编号 as id, 分类编号 as cat_id, 文件特征 as file_sha, 描述 as desc
"#,
		&user.id,
		body.cat_id,
		body.file_sha,
		body.desc,
	)
	.fetch_one(&state)
	.await?;

	Ok(Json(emoji))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListBody {
	cat_id: Option<i32>,
	search_word: Option<String>,
}

#[post("/list")]
async fn list_emoji(
	user: User,
	body: Json<ListBody>,
	state: EmojiState,
) -> Result<Json<Vec<Emoji>>> {
	let mut qb = sqlx::QueryBuilder::<sqlx::Postgres>::new(
		r#"
select 编号 as id, 分类编号 as cat_id, 文件特征 as file_sha, 描述 as desc
from 表情
where"#,
	);

	qb.push(" 用户编号 = ");
	qb.push_bind(&user.id);

	qb.push(" and 分类编号 is not distinct from ");
	qb.push_bind(&body.cat_id);

	if let Some(search_word) = &body.search_word {
		qb.push(" and to_tsvector('china', 描述) @@ to_tsquery('china', ");
		qb.push_bind(search_word);
		qb.push(")");
	}

	qb.push(" order by 编号 desc");

	let emojis = qb.build_query_as::<Emoji>().fetch_all(&state).await?;

	Ok(Json(emojis))
}

#[derive(Debug, Deserialize)]
struct UpdateDescBody {
	desc: String,
}

#[post("/update/desc")]
async fn update_emoji_desc(
	user: User,
	body: Json<UpdateBody<UpdateDescBody>>,
	state: EmojiState,
) -> Result<Json<Emoji>> {
	sqlx::query_scalar!(
		r#"select 1 from 表情 where 编号 = $1 and 用户编号 = $2 limit 1"#,
		&body.id,
		&user.id
	)
	.fetch_optional(&state)
	.await?
	.ok_or(Error::not_found("表情不存在！"))?;

	let emoji = sqlx::query_as!(
		Emoji,
		r#"
update 表情
set 描述 = $1
where 编号 = $2
returning 编号 as id, 描述 as desc, 分类编号 as cat_id, 文件特征 as file_sha
"#,
		&body.data.desc,
		&body.id
	)
	.fetch_one(&state)
	.await?;

	Ok(Json(emoji))
}

#[derive(Deserialize)]
struct DeleteBody {
	id: i32,
}

#[post("/delete")]
async fn remove_emoji(user: User, body: Json<DeleteBody>, state: EmojiState) -> Result<Json<()>> {
	sqlx::query_scalar!(
		r#"
delete from 表情
where 表情.编号 = $1 and 用户编号 = $2
"#,
		&body.id,
		&user.id,
	)
	.fetch_optional(&state)
	.await?;

	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/emoji")
		.service(create_emoji)
		.service(list_emoji)
		.service(update_emoji_desc)
		.service(remove_emoji)
}
