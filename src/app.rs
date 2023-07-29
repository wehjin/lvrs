use std::error::Error;
use crate::app::AppKey::UsingEmoji;
use crate::lv::{LiveView, Session, Assigns, Value, Vision};
use crate::lv::socket::Socket;

pub struct AppParams {}

pub enum AppMsg {
	Toggle
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppKey {
	UsingEmoji,
}

pub struct App {}

impl LiveView for App {
	type Params = AppParams;
	type Msg = AppMsg;
	type StateKey = AppKey;

	fn render(assigns: &Assigns<Self::StateKey>) -> Vision {
		let using_emoji = assigns.read(&UsingEmoji).to_bool();
		match using_emoji {
			true => Vision::LiveEmoji,
			false => Vision::LiveText,
		}
	}

	fn mount(_params: &Self::Params, _session: &Session, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		let socket = socket.assign(UsingEmoji, Value::Bool(true));
		Ok(socket)
	}

	fn handle_event(msg: Self::Msg, _params: &Self::Params, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		match msg {
			AppMsg::Toggle => {
				let socket = socket.update(UsingEmoji, Value::flip);
				Ok(socket)
			}
		}
	}
}

