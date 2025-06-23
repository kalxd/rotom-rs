use ntex::web;

mod data;

#[web::get("/")]
async fn index() -> impl web::Responder {
	"hello world"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
	web::HttpServer::new(|| web::App::new().service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}
