use ntex::web;

mod data;

use data::error::{Error, Result};

#[web::get("/")]
async fn index() -> impl web::Responder {
	"hello world"
}

#[ntex::main]
async fn main() -> Result<()> {
	web::HttpServer::new(|| web::App::new().service(index))
		.bind(("127.0.0.1", 8080))
		.map_err(Error::internal)?
		.run()
		.await
		.map_err(Error::internal)
}
