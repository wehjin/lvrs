pub(crate) mod assets;
pub(crate) mod app_state;
pub(crate) mod app_ws;

use std::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws;
use crate::lv::server::app_state::AppState;
use crate::lv::server::app_ws::AppWs;

#[get("/rs")]
async fn index(app_state: web::Data<AppState>) -> impl Responder {
	match app_state.html_string().await {
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
async fn live_websocket(req: HttpRequest, stream: web::Payload, app_state: web::Data<AppState>) -> impl Responder {
	let app_worker = AppWs::new(app_state.app_worker().clone());
	let resp = ws::start(app_worker, &req, stream);
	resp
}

pub async fn start() -> Result<(), Box<dyn Error>> {
	let host = "127.0.0.1";
	let port = 8000;
	println!("Listening at http://{}:{}/rs", host, port);
	HttpServer::new(|| {
		let app = App::new()
			.app_data(web::Data::new(AppState::new()))
			.service(index)
			.service(index_js)
			.service(index_js_map)
			.service(live_websocket);
		app
	}).bind((host, port))?.run().await?;
	Ok(())
}


