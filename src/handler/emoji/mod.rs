use ntex::web::{DefaultError, Scope, post, scope, types::Json};
use serde::{Deserialize, Serialize};

use crate::data::{
	User,
	error::{Error, Result},
};
use crate::helper;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBody {
	file_id: String,
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
			user_id,
			cat_id
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
	file_id: String,
	desc: Option<String>,
}

#[post("/create")]
async fn create_emoji(
	user: User,
	body: Json<CreateBody>,
	state: EmojiState,
) -> Result<Json<Emoji>> {
	state.file.check_file_by_sha(&body.file_id).await?;

	if let Some(ref cat_id) = body.cat_id {
		state.check_user_cat(&user.id, cat_id).await?;
	}

	let emoji = sqlx::query_as!(
		Emoji,
		r#"
insert into 表情
(分类编号, 文件编号, 描述)
values ($1, $2, $3)
returning 编号 as id, 分类编号 as cat_id, 文件编号 as file_id, 描述 as desc
"#,
		body.cat_id,
		body.file_id,
		body.desc,
	)
	.fetch_one(&state)
	.await?;

	Ok(Json(emoji))
}

pub fn api() -> Scope<DefaultError> {
	scope("/user/emoji")
}
