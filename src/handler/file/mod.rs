use ntex::web::{DefaultError, Scope, scope};

pub fn api() -> Scope<DefaultError> {
	scope("/file")
}
