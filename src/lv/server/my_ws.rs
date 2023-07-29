use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use crate::app::{App, AppKey, AppMsg, AppParams};
use crate::lv::phx::{PhxMsgType, PHX_REPLY};
use crate::lv::phx;
use serde_json::Value as JsonValue;
use crate::lv::{LiveView, Session, Vision};
use crate::lv::socket::Socket;

pub(crate) struct MyWs {
	params: AppParams,
	session: Session,
	socket: Socket<AppKey>,
	vision: Vision,
}

impl MyWs {
	pub fn new() -> Self {
		let params = AppParams {};
		let session = Session {};
		let socket = App::mount(&params, &session, &Socket::new()).expect("mount");
		let vision = App::render(&socket.assigns());
		MyWs { params, session, socket, vision }
	}

	fn handle_event(&mut self, msg: &JsonValue) -> Option<String> {
		match msg {
			JsonValue::Array(vec) => {
				let msg_type = PhxMsgType::from(&vec[3]);
				let response = match msg_type {
					PhxMsgType::Join => Some(self.vision.phx_rendered()),
					PhxMsgType::Heartbeat => Some(json!({})),
					PhxMsgType::Event => {
						println!("ASSIGNS BEFORE: {:?}", self.socket.assigns());
						self.socket = App::handle_event(AppMsg::Toggle, &self.params, &self.socket).expect("handle_event");
						let new_vision = App::render(&self.socket.assigns());
						let phx_diffs = self.vision.phx_diffs(&new_vision);
						self.vision = new_vision;
						println!("ASSIGNS AFTER: {:?}", self.socket.assigns());
						Some(phx_diffs)
					}
					PhxMsgType::Unknown(_) => None,
				};
				let reply = response.map(|response| json!([
					vec[0].clone(),
					vec[1].clone(),
					vec[2].clone(),
					PHX_REPLY,
					phx::reply_ok(response)
				]));
				let reply = reply.map(|it| it.to_string());
				reply
			}
			_ => None
		}
	}
}

impl Actor for MyWs {
	type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		println!("WEBSOCKET MSG: {:?}", msg);
		match msg {
			Ok(ws::Message::Text(text)) => {
				let json: JsonValue = serde_json::from_str(&text.to_string()).unwrap();
				let reply = self.handle_event(&json);
				println!("WEBSOCKET REPLY: {:?}", reply);
				if let Some(reply) = reply {
					ctx.text(reply);
				} else {
					ctx.text(text);
				}
			}
			Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
			Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
			_ => (),
		}
	}
}


