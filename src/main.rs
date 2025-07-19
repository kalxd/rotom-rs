use ntex::web;

mod data;
mod handler;
mod helper;

use data::{AppState, config::load_config, error::Result};

#[ntex::main]
async fn main() -> Result<()> {
	println!("start it!!!!");

	data::file::ensure_base_dir()?;

	let config = load_config()?;
	let state = AppState::from_config(&config).await?;

	web::HttpServer::new(move || web::App::new().state(state.clone()).service(handler::api()))
		.bind(("0.0.0.0", 3000))?
		.run()
		.await?;

	Ok(())
}
