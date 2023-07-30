use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message};
use serde_json::Value as JsonValue;
use crate::lv::app::agent::{AppAgent, AppAgentMsg};

#[derive(actix::Message)]
#[rtype(result = "()")]
pub(crate) enum MyWsMsg {
	PhxReply(String)
}

pub(crate) struct AppWs {
	app_agent: Addr<AppAgent>,
}

impl Actor for AppWs {
	type Context = ws::WebsocketContext<Self>;
}

impl Handler<MyWsMsg> for AppWs {
	type Result = ();

	fn handle(&mut self, msg: MyWsMsg, ctx: &mut Self::Context) -> Self::Result {
		match msg {
			MyWsMsg::PhxReply(reply) => ctx.text(reply),
		}
	}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AppWs {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		println!("WEBSOCKET MSG: {:?}", msg);
		match msg {
			Ok(msg) => {
				match msg {
					Message::Text(text) => {
						let request: JsonValue = serde_json::from_str(&text.to_string()).unwrap();
						let replier = ctx.address().recipient();
						self.app_agent.do_send(AppAgentMsg::PhxRequest { request, requester: replier });
					}
					Message::Binary(bin) => ctx.binary(bin),
					Message::Continuation(_) => ctx.stop(),
					Message::Ping(msg) => ctx.pong(&msg),
					Message::Pong(_msg) => {}
					Message::Close(reason) => {
						ctx.close(reason);
						ctx.stop()
					}
					Message::Nop => ()
				}
			}
			Err(_) => ()
		}
	}
}

impl AppWs {
	pub fn new(app_worker: Addr<AppAgent>) -> Self {
		AppWs { app_agent: app_worker }
	}
}


