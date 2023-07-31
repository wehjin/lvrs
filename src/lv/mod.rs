pub mod prelude {
	pub type JsonValue = serde_json::Value;
}

pub mod server;
pub mod app;
mod value;
mod state;
pub(crate) mod phx;
pub(crate) mod vision;

use std::error::Error;
use std::hash::Hash;

pub use value::*;
pub use state::*;
use crate::lv::app::socket::Socket;
use crate::lv::vision::Vision;

pub struct Session {}

pub trait LiveView {
	type Params;
	type Msg;
	type AssignKeys: Eq + Hash + Clone;

	fn mount(params: &Self::Params, session: &Session, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>>;
	fn handle_event(msg: Self::Msg, params: &Self::Params, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>>;
	fn render(state: &Assigns<Self::AssignKeys>) -> Vision;
}


