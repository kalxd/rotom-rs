use ntex::web;
use ralts::error::Result;

mod data;
mod handler;
mod helper;

use data::{AppState, config};

#[ntex::main]
async fn main() -> Result<()> {
	println!("start it!!!!");

	data::file::ensure_base_dir()?;

	let config = config::Config::load_config()?;
	let state = AppState::from_config(&config).await?;

	web::HttpServer::new(async move || {
		web::App::new().state(state.clone()).service(handler::api())
	})
	.bind(("0.0.0.0", 3000))?
	.run()
	.await?;

	Ok(())
}
