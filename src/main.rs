use ntex::web;

mod data;

use data::{
	AppState,
	config::load_config,
	error::{Error, Result},
};

#[web::get("/")]
async fn index() -> impl web::Responder {
	"hello world"
}

#[ntex::main]
async fn main() -> Result<()> {
	let config = load_config()?;
	let state = AppState::from_config(&config).await?;

	web::HttpServer::new(move || web::App::new().state(state.clone()).service(index))
		.bind(("127.0.0.1", 3000))
		.map_err(Error::internal)?
		.run()
		.await
		.map_err(Error::internal)
}
