use ntex::web;

mod data;
mod handler;

use data::{
	AppState,
	config::load_config,
	error::{Error, Result},
};

#[ntex::main]
async fn main() -> Result<()> {
	let config = load_config()?;
	let state = AppState::from_config(&config).await?;

	web::HttpServer::new(move || web::App::new().state(state.clone()).service(handler::api()))
		.bind(("0.0.0.0", 3000))
		.map_err(Error::internal)?
		.run()
		.await
		.map_err(Error::internal)
}
