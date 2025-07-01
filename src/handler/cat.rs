use ntex::web::{DefaultError, Scope, get, scope, types::Json};

use crate::data::{User, error::Result};

#[get("/list")]
async fn get_all_cat(user: User) -> Result<Json<()>> {
	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/self/cat").service(get_all_cat)
}
