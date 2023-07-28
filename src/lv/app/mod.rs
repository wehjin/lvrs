use std::error::Error;
use actix::Addr;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::lv::{socket};
use crate::lv::actor::{HtmlAsString, SocketActor};

struct LiveApp {
	socket: Addr<SocketActor>,
}

impl LiveApp {
	pub fn new() -> Self {
		let socket = socket::actor::start();
		LiveApp { socket }
	}
	pub async fn html_string(&self) -> Result<String, Box<dyn Error>> {
		let live_slice = self.socket.send(HtmlAsString).await?;
		let prefix = assets::prefix();
		let postfix = assets::postfix();
		Ok(format!("{}{}{}", prefix, live_slice, postfix))
	}
}

#[get("/rs")]
async fn index(live_app: web::Data<LiveApp>) -> impl Responder {
	let html_string = live_app.html_string().await;
	match html_string {
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

pub async fn start() -> Result<(), Box<dyn Error>> {
	HttpServer::new(|| {
		let live_app = LiveApp::new();
		let app = App::new()
			.app_data(web::Data::new(live_app))
			.service(index)
			.service(index_js)
			.service(index_js_map)
			.route("/live/websocket", web::get().to(websocket::ws_index));
		app
	}).bind(("127.0.0.1", 8000))?.run().await?;
	Ok(())
}

pub(crate) mod websocket;
pub(crate) mod assets;