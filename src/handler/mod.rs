use ntex::web::{ErrorRenderer, Scope, scope};

pub fn api<E: ErrorRenderer>() -> Scope<E> {
	scope("/")
}
