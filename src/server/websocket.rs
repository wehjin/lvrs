use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message};
use serde_json::Value as JsonValue;
use crate::live::agent::{LiveAgent, LiveAgentMsg};
use crate::LiveView;

#[derive(actix::Message)]
#[rtype(result = "()")]
pub(crate) enum WebsocketMsg { PhxReply(JsonValue) }

pub(crate) struct Websocket<T: LiveView + 'static> {
	live_link: Addr<LiveAgent<T>>,
}

impl<T: LiveView + 'static> Actor for Websocket<T> {
	type Context = ws::WebsocketContext<Self>;
}

impl<T: LiveView + 'static> Handler<WebsocketMsg> for Websocket<T> {
	type Result = ();

	fn handle(&mut self, msg: WebsocketMsg, ctx: &mut Self::Context) -> Self::Result {
		match msg {
			WebsocketMsg::PhxReply(reply) => {
				eprintln!("WEBSOCKET REPLY: {}", &reply);
				ctx.text(reply.to_string())
			}
		}
	}
}

impl<T: LiveView + 'static> StreamHandler<Result<ws::Message, ws::ProtocolError>> for Websocket<T>

{
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		eprintln!("---\nWEBSOCKET MSG: {:?}", msg);
		match msg {
			Ok(msg) => {
				match msg {
					Message::Text(text) => {
						let request: JsonValue = serde_json::from_str(&text.to_string()).unwrap();
						let replier = ctx.address().recipient();
						self.live_link.do_send(LiveAgentMsg::PhxRequest { request, requester: replier });
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

impl<T: LiveView + 'static> Websocket<T> {
	pub fn new(live_agent: Addr<LiveAgent<T>>) -> Self {
		Websocket { live_link: live_agent }
	}
}


