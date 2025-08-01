use ntex::web::{DefaultError, Scope, get, scope, types::Json};

use crate::data::{User, error::Result};

mod cat;
mod emoji;

#[get("/info")]
async fn self_info(user: Option<User>) -> Result<Json<Option<User>>> {
	Ok(Json(user))
}

pub fn api() -> Scope<DefaultError> {
	scope("/self")
		.service(self_info)
		.service(cat::api())
		.service(emoji::api())
}
