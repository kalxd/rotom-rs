use ntex::web::{DefaultError, Scope, scope};

mod cat;
mod file;
mod user;

pub fn api() -> Scope<DefaultError> {
	scope("/")
		.service(user::api())
		.service(cat::api())
		.service(file::api())
}
