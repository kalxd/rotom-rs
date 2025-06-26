use ntex::web::{DefaultError, Scope, scope};

mod user;

pub fn api() -> Scope<DefaultError> {
	scope("/").service(user::api())
}
