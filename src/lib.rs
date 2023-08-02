#[macro_use]
extern crate serde_json;

pub mod prelude {
	pub type JsonValue = serde_json::Value;
}

#[macro_use]
pub mod vision;
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

	fn mount(params: &Self::Params, session: &Session, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>>;
	fn handle_event(msg: Self::Msg, params: &Self::Params, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>>;
	fn render(state: &Assigns<Self::AssignKeys>) -> Vision;
}


