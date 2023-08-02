#[macro_use]
extern crate serde_json;

#[macro_use]
pub mod vision;

pub mod prelude {
	pub type JsonValue = serde_json::Value;
}

pub mod server;
pub mod live;
mod value;
mod state;
pub(crate) mod phx;

use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub use value::*;
pub use state::*;
use crate::live::socket::Socket;
use crate::vision::Vision;

pub struct Session {}

pub trait LiveMsg: Sized {
	type Err: Display;
	fn from_str(s: impl AsRef<str>) -> Result<Self, Self::Err>;
}

pub trait LiveView {
	type Params: Unpin;
	type Msg: LiveMsg;
	type AssignKeys: Debug + Clone + Eq + Hash + Unpin;

	fn mount(_params: &Self::Params, _session: &Session, socket: Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		Ok(socket)
	}
	fn handle_event(_msg: Self::Msg, _params: &Self::Params, socket: Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		Ok(socket)
	}
	fn render(state: &Assigns<Self::AssignKeys>) -> Vision;
}


