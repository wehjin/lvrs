pub(crate) mod assets;
pub(crate) mod websocket;

use std::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws as actix_ws;
use crate::live::link::LiveLink;
use crate::LiveView;
use crate::server::websocket::Websocket;

#[derive(Debug, Clone)]
pub struct Route(String);

impl Route {
	pub fn new(s: impl AsRef<str>) -> Self {
		Route(s.as_ref().to_string())
	}
	pub fn path(&self) -> &str {
		&self.0
	}
	pub fn to_url(&self, host: impl AsRef<str>, port: u16) -> String {
		format!("http://{}:{}{}", host.as_ref(), port, self.0)
	}
}

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8000;
const WEBSOCKET_PATH: &str = "/live/websocket";

pub async fn start<T: LiveView + 'static>(params: T::Params, route: Route) -> Result<(), Box<dyn Error>>
{
	let live_app = web::Data::new(LiveLink::<T>::start(params));
	let route_url = route.to_url(HOST, PORT);
	println!("Listening at {}", route_url);
	HttpServer::new(move || {
		App::new()
			.app_data(live_app.clone())
			.service(index_js)
			.service(index_js_map)
			.service(web::resource(route.path()).route(web::get().to(index::<T>)))
			.service(web::resource(WEBSOCKET_PATH).route(web::get().to(live_websocket::<T>)))
	}).bind((HOST, PORT))?.run().await?;
	Ok(())
}

async fn index<T: LiveView + 'static>(live_link: web::Data<LiveLink<T>>) -> impl Responder {
	match live_link.html_string().await {
		Ok(html) => HttpResponse::Ok().body(html),
		Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
	}
}

async fn live_websocket<T: LiveView + 'static>(req: HttpRequest, stream: web::Payload, live_link: web::Data<LiveLink<T>>) -> impl Responder {
	let live_ws = Websocket::new(live_link.as_agent().clone());
	actix_ws::start(live_ws, &req, stream)
}

#[get("/index.js")]
async fn index_js() -> impl Responder {
	let s = include_str!("assets/index.js");
	HttpResponse::Ok().body(s)
}

#[get("/index.js.map")]
async fn index_js_map() -> impl Responder {
	let s = include_str!("assets/index.js.map");
	HttpResponse::Ok().body(s)
}
