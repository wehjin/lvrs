use actix::prelude::*;
use actix::Message;
use crate::lv::prelude::*;
use crate::lv::{LiveView, Session};
use crate::sample::{SampleApp, SampleAppAssignKey, SampleAppMsg, SampleAppParams};
use crate::lv::app::socket::Socket;
use crate::lv::phx::{PhxEvent};
use crate::lv::server::app_ws::AppWsMsg;
use crate::lv::vision::{Vision};

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct HtmlAsString;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum AppAgentMsg {
	PhxRequest { request: JsonValue, requester: Recipient<AppWsMsg> }
}

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct ToHtmlString;

pub(crate) struct AppAgent {
	params: SampleAppParams,
	session: Session,
	socket: Socket<SampleAppAssignKey>,
	vision: Vision,
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
					requester.do_send(AppWsMsg::PhxReply(reply));
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
	fn handle_event(&mut self, msg: &JsonValue) -> Option<JsonValue> {
		let phx = PhxEvent::from_array(msg.as_array().unwrap());
		println!("PHX EVENT: {:?}", phx);
		match &phx {
			PhxEvent::Heartbeat(_) => {
				let json = json!({});
				Some(phx.context().to_phx_reply(json))
			}
			PhxEvent::Join(_, _) => {
				let json = json!({"rendered": self.vision.to_phx_rendered()});
				Some(phx.context().to_phx_reply(json))
			}
			PhxEvent::UserEvent { event, .. } => {
				if event == "toggle" {
					println!("ASSIGNS BEFORE: {:?}", self.socket.assigns());
					self.socket = SampleApp::handle_event(SampleAppMsg::Toggle, &self.params, &self.socket).expect("handle_event");
					println!("ASSIGNS AFTER: {:?}", self.socket.assigns());
					let (json, new_vision) = {
						let new_vision = SampleApp::render(&self.socket.assigns());
						let json = json!({"diff": self.vision.to_phx_diff(&new_vision)});
						(json, new_vision)
					};
					self.vision = new_vision;
					Some(phx.context().to_phx_reply(json))
				} else {
					None
				}
			}
			PhxEvent::NotImplemented(_, _, _) => None
		}
	}
}

