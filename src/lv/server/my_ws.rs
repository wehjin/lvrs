use std::collections::HashMap;
use std::ops::Not;
use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use crate::app::AppKey;
use crate::lv::phx::PhxMsgType;
use serde_json::Value;
use crate::lv;

pub(crate) struct MyWs {
	assigns: HashMap<AppKey, lv::Value>,
}

impl MyWs {
	pub fn new() -> Self {
		let assigns = vec![(AppKey::UsingEmoji, lv::Value::from(true))].into_iter().collect();
		MyWs { assigns }
	}
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
				let reply = self.handle_event(&json);
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
	fn handle_event(&mut self, msg: &Value) -> Option<String> {
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
						println!("ASSIGNS BEFORE: {:?}", self.assigns);
						let using_emoji = self.assigns.get(&AppKey::UsingEmoji).expect("using-emoji").to_bool();
						let json = if using_emoji {
							json!({"diff":{"0":"Hello","2":"Use Emoji"}})
						} else {
							json!({"diff":{"0":"👋","2":"Use Text"}})
						};
						self.assigns.insert(AppKey::UsingEmoji, lv::Value::from(!using_emoji));
						println!("ASSIGNS AFTER: {:?}", self.assigns);
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


