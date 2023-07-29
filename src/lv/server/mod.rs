use std::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws;
use crate::lv::server::app::LiveApp;
use crate::lv::server::my_ws::MyWs;

#[get("/rs")]
async fn index(live_app: web::Data<LiveApp>) -> impl Responder {
	match live_app.html_string().await {
		Ok(html) => HttpResponse::Ok().body(html),
		Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
	}
}

#[get("/index.js")]
async fn index_js() -> impl Responder {
	let s = include_str!("index.js");
	HttpResponse::Ok().body(s)
}

#[get("/index.js.map")]
async fn index_js_map() -> impl Responder {
	let s = include_str!("index.js.map");
	HttpResponse::Ok().body(s)
}

#[get("/live/websocket")]
async fn live_websocket(req: HttpRequest, stream: web::Payload) -> impl Responder {
	let actor = MyWs::new();
	let resp = ws::start(actor, &req, stream);
	println!("WEBSOCKET: {:?}", resp);
	resp
}

pub async fn start() -> Result<(), Box<dyn Error>> {
	let host = "127.0.0.1";
	let port = 8000;
	println!("Listening at http://{}:{}/rs", host, port);
	HttpServer::new(|| {
		let live_app = LiveApp::new();
		let app = App::new()
			.app_data(web::Data::new(live_app))
			.service(index)
			.service(index_js)
			.service(index_js_map)
			.service(live_websocket);
		app
	}).bind((host, port))?.run().await?;
	Ok(())
}

pub(crate) mod assets;
pub(crate) mod app;
pub(crate) mod my_ws;
