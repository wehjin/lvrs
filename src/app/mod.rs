use std::error::Error;
use crate::app::AppKey::UsingEmoji;
use crate::lv::{LiveView, Session, Socket, State, Vision};

pub struct AppParams {}

pub enum AppMsg {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AppKey { UsingEmoji }

pub struct App {}

impl LiveView for App {
	type Params = AppParams;
	type Msg = AppMsg;
	type StateKey = AppKey;

	fn render(_state: &State<Self::StateKey>) -> Vision {
		Vision::Live
	}

	fn mount(_params: &Self::Params, _session: &Session, socket: Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		let socket = socket.assign(UsingEmoji, true.into());
		Ok(socket)
	}
}

