use actix::prelude::*;
use actix::Message;
use crate::prelude::*;
use crate::{LiveMsg, LiveView, Session};
use crate::live::socket::Socket;
use crate::phx::{PhxEvent};
use crate::server::websocket::WebsocketMsg;
use crate::vision::{Vision};

pub(crate) fn start<T: LiveView + 'static>(params: T::Params) -> Addr<LiveAgent<T>> {
	let actor = LiveAgent::new(params);
	actor.start()
}

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct HtmlAsString;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum LiveAgentMsg {
	PhxRequest { request: JsonValue, requester: Recipient<WebsocketMsg> }
}

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct ToHtmlString;

pub(crate) struct LiveAgent<T: LiveView + 'static> {
	params: T::Params,
	session: Session,
	socket: Option<Socket<T::AssignKeys>>,
	vision: Vision,
}

impl<T: LiveView + 'static> Actor for LiveAgent<T>
{
	type Context = Context<Self>;
}

impl<T: LiveView + 'static> Handler<LiveAgentMsg> for LiveAgent<T> {
	type Result = ();

	fn handle(&mut self, msg: LiveAgentMsg, _ctx: &mut Self::Context) -> Self::Result {
		match msg {
			LiveAgentMsg::PhxRequest { request, requester } => {
				let reply = self.handle_event(&request);
				if let Some(reply) = reply {
					requester.do_send(WebsocketMsg::PhxReply(reply));
				}
			}
		}
	}
}

impl<T: LiveView + 'static> Handler<ToHtmlString> for LiveAgent<T> {
	type Result = String;

	fn handle(&mut self, _msg: ToHtmlString, _ctx: &mut Self::Context) -> Self::Result {
		self.vision.to_html_string()
	}
}

impl<T: LiveView + 'static> LiveAgent<T>
{
	pub fn new(params: T::Params) -> Self {
		let session = Session {};
		let socket = T::mount(&params, &session, Socket::new()).expect("mount");
		let vision = T::render(&socket.assigns());
		LiveAgent { params, session, socket: Some(socket), vision }
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
				// TODO Check csrf token and start live-view agent.
				let json = json!({"rendered": self.vision.to_phx_rendered()});
				Some(phx.context().to_phx_reply(json))
			}
			PhxEvent::UserEvent { event, .. } => {
				let msg = T::Msg::from_str(event);
				match msg {
					Ok(msg) => {
						let socket = self.socket.take().unwrap();
						println!("ASSIGNS BEFORE: {:?}", socket.assigns());
						let new_socket = T::handle_event(msg, &self.params, socket).expect("handle_event");
						println!("ASSIGNS AFTER: {:?}", new_socket.assigns());
						let (json, new_vision) = {
							let new_vision = T::render(&new_socket.assigns());
							let json = json!({"diff": self.vision.to_phx_diff(&new_vision)});
							(json, new_vision)
						};
						self.socket = Some(new_socket);
						self.vision = new_vision;
						Some(phx.context().to_phx_reply(json))
					}
					Err(e) => {
						eprintln!("{}", e);
						None
					}
				}
			}
			PhxEvent::NotImplemented(_, _, _) => None
		}
	}
}

