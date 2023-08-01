pub(crate) mod assets;
pub(crate) mod websocket;

use std::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws as actix_ws;
use crate::lv::live::link::LiveLink;
use crate::lv::LiveView;
use crate::lv::server::websocket::Websocket;
use crate::sample::{SampleApp};

pub(crate) async fn start<T: LiveView + 'static>(params: T::Params) -> Result<(), Box<dyn Error>>
{
	let host = "127.0.0.1";
	let port = 8000;
	println!("Listening at http://{}:{}/rs", host, port);
	let live_app = web::Data::new(LiveLink::<T>::start(params));
	HttpServer::new(move || {
		let app = App::new()
			.app_data(live_app.clone())
			.service(index)
			.service(index_js)
			.service(index_js_map)
			.service(live_websocket);
		app
	}).bind((host, port))?.run().await?;
	Ok(())
}

#[get("/index.js")]
async fn index_js() -> impl Responder {
	let s = include_str!("index.js");
	HttpResponse::Ok().body(s)
}

#[get("/rs")]
async fn index(live_link: web::Data<LiveLink<SampleApp>>) -> impl Responder {
	// TODO Remove SampleApp from signature
	match live_link.html_string().await {
		Ok(html) => HttpResponse::Ok().body(html),
		Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
	}
}

#[get("/index.js.map")]
async fn index_js_map() -> impl Responder {
	let s = include_str!("index.js.map");
	HttpResponse::Ok().body(s)
}

#[get("/live/websocket")]
async fn live_websocket(req: HttpRequest, stream: web::Payload, live_link: web::Data<LiveLink<SampleApp>>) -> impl Responder {
	// TODO Remove SampleApp from signature
	let live_ws = Websocket::new(live_link.as_agent().clone());
	let resp = actix_ws::start(live_ws, &req, stream);
	resp
}
