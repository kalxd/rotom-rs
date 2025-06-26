use ntex::web::{DefaultError, Responder, Scope, post, scope};

#[post("/login")]
async fn login() -> impl Responder {
	"hello world"
}

pub fn api() -> Scope<DefaultError> {
	scope("/user").service(login)
}
