use ntex::web::{
	DefaultError, Scope, post, scope,
	types::{Json, State},
};
use serde::Deserialize;

use crate::data::{AppState, error::Result, ty::SaltPassword};

#[derive(Debug, Deserialize)]
struct LoginBody {
	username: String,
	password: String,
}

#[post("/login")]
async fn login(body: Json<LoginBody>, state: State<AppState>) -> Result<Json<()>> {
	let salt = SaltPassword::new(&body.password, &state.salt);

	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/user").service(login)
}
