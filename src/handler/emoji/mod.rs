use ntex::web::{
	DefaultError, Scope, post, scope,
	types::{Json, State},
};
use serde::Deserialize;

use crate::data::{AppState, User, error::Result};

#[derive(Debug, Deserialize)]
struct CreateBody {
	fileId: String,
	catId: Option<i32>,
	desc: Option<String>,
}

#[post("/create")]
async fn create_emoji(
	user: User,
	body: Json<CreateBody>,
	state: State<AppState>,
) -> Result<Json<()>> {
	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/user/emoji")
}
