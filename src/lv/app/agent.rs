use actix::prelude::*;
use actix::Message;
use crate::lv::prelude::*;
use crate::lv::{LiveView, Session, FakeVision};
use crate::sample::{SampleApp, SampleAppAssignKey, SampleAppMsg, SampleAppParams};
use crate::lv::app::socket::Socket;
use crate::lv::phx::{PhxMsgType, PHX_REPLY, reply_ok};
use crate::lv::server::app_ws::MyWsMsg;

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct HtmlAsString;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum AppAgentMsg {
	PhxRequest { request: JsonValue, requester: Recipient<MyWsMsg> }
}

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct ToHtmlString;

pub(crate) struct AppAgent {
	params: SampleAppParams,
	session: Session,
	socket: Socket<SampleAppAssignKey>,
	vision: FakeVision,
}

impl Actor for AppAgent {
	type Context = Context<Self>;
}

impl Handler<AppAgentMsg> for AppAgent {
	type Result = ();

	fn handle(&mut self, msg: AppAgentMsg, _ctx: &mut Self::Context) -> Self::Result {
		match msg {
			AppAgentMsg::PhxRequest { request, requester } => {
				let reply = self.handle_event(&request);
				if let Some(reply) = reply {
					let msg = MyWsMsg::PhxReply(reply);
					requester.do_send(msg);
				}
			}
		}
	}
}

impl Handler<ToHtmlString> for AppAgent {
	type Result = String;

	fn handle(&mut self, _msg: ToHtmlString, _ctx: &mut Self::Context) -> Self::Result {
		self.vision.to_html_string()
	}
}

pub(crate) fn start() -> Addr<AppAgent> {
	let actor = AppAgent::new();
	actor.start()
}

impl AppAgent {
	pub fn new() -> Self {
		let params = SampleAppParams {};
		let session = Session {};
		let socket = SampleApp::mount(&params, &session, &Socket::new()).expect("mount");
		let vision = SampleApp::render(&socket.assigns());
		AppAgent { params, session, socket, vision }
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
						self.socket = SampleApp::handle_event(SampleAppMsg::Toggle, &self.params, &self.socket).expect("handle_event");
						let new_vision = SampleApp::render(&self.socket.assigns());
						let phx_diffs = self.vision.phx_diff(&new_vision);
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
					reply_ok(response)
				]));
				let reply = reply.map(|it| it.to_string());
				reply
			}
			_ => None
		}
	}
}

