use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web_actors::ws;

#[macro_use]
extern crate serde_json;

#[get("/")]
async fn hello() -> impl Responder {
	let s = include_str!("hello.html");
	HttpResponse::Ok().body(s)
}

#[get("/index.js")]
async fn index_js() -> impl Responder {
	let s = include_str!("index.js");
	HttpResponse::Ok().body(s)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		let app = App::new()
			.service(hello)
			.service(index_js)
			.service(echo)
			.route("/hey", web::get().to(manual_hello))
			.route("/live/websocket", web::get().to(ws_index));
		app
	}).bind(("127.0.0.1", 8000))?.run().await
}

use actix::{Actor, StreamHandler};
use serde_json::Value;
use crate::protocol::PhxMsgType;

struct MyWs {
	using_emoji: bool,
}

impl Actor for MyWs {
	type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		println!("{:?}", msg);
		match msg {
			Ok(ws::Message::Ping(msg)) => {
				ctx.pong(&msg)
			}
			Ok(ws::Message::Text(text)) => {
				let json: Value = serde_json::from_str(&text.to_string()).unwrap();
				let reply = self.to_reply(&json);
				println!("{:?}", reply);
				if let Some(reply) = reply {
					ctx.text(reply);
				} else {
					ctx.text(text);
				}
			}
			Ok(ws::Message::Binary(bin)) => {
				ctx.binary(bin)
			}
			_ => {
				()
			}
		}
	}
}

impl MyWs {
	fn to_reply(&mut self, msg: &Value) -> Option<String> {
		match msg {
			Value::Array(vec) => {
				let msg_type = PhxMsgType::from(&vec[3]);
				let response = match msg_type {
					PhxMsgType::Join => {
						let json = json!({
							"rendered":{
								"0":"👋",
								"1":"Liveviewjstest",
								"2":"Use Text",
								"s":["\n      <div class=\"flex flex-col items-center space-y-10 pt-10\">\n        <div class=\"flex flex-col items-center space-y-5\">\n          <h1 class=\"text-2xl font-bold\">"," ","</h1>\n          <button class=\"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded\" phx-click=\"toggle\">\n            ","\n          </button>\n        </div>\n        <div class=\"text-center max-w-[200px]\">\n          More documentation and examples at\n          <a class=\"text-blue-500\" href=\"https://liveviewjs.com\" target=\"_blank\" rel=\"noopener noreferrer\"\n            >LiveViewJS.com</a\n          >\n        </div>\n      </div>\n    "]
							}
						});
						Some(json)
					}
					PhxMsgType::Heartbeat => {
						let json = json!({});
						Some(json)
					}
					PhxMsgType::Event => {
						let json = if self.using_emoji {
							json!({"diff":{"0":"Hello","2":"Use Emoji"}})
						} else {
							json!({"diff":{"0":"👋","2":"Use Text"}})
						};
						self.using_emoji = !self.using_emoji;
						Some(json)
					}
					PhxMsgType::Unknown(_) => None,
				};
				let reply = response.map(|response| json!([
				vec[0].clone(),
				vec[1].clone(),
				vec[2].clone(),
				"phx_reply",
				{
					"status":"ok",
					"response":response
				}
			]));
				let reply = reply.map(|it| it.to_string());
				reply
			}
			_ => None
		}
	}
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
	let resp = ws::start(MyWs { using_emoji: true }, &req, stream);
	println!("{:?}", resp);
	resp
}

pub mod protocol {
	use serde_json::Value;
	use crate::protocol::PhxMsgType::{Event, Heartbeat, Join};

	#[derive(Debug, Clone)]
	pub enum PhxMsgType {
		Join,
		Heartbeat,
		Event,
		Unknown(String),
	}

	impl From<&Value> for PhxMsgType {
		fn from(value: &Value) -> Self {
			let s = value.as_str().unwrap();
			match s {
				"phx_join" => Join,
				"heartbeat" => Heartbeat,
				"event" => Event,
				&_ => PhxMsgType::Unknown(s.to_string())
			}
		}
	}
}