use ntex::web::{DefaultError, Scope, get, scope, types::Json};

use crate::data::{User, error::Result};

#[get("/self")]
async fn self_info(user: Option<User>) -> Result<Json<Option<User>>> {
	Ok(Json(user))
}

pub fn api() -> Scope<DefaultError> {
	scope("/user").service(self_info)
}
