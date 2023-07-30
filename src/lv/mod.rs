pub mod prelude {
	pub type JsonValue = serde_json::Value;
}

pub mod server;
pub mod app;
mod value;
mod state;
pub(crate) mod phx;
pub(crate) mod vision2;

use std::error::Error;
use std::hash::Hash;

pub use value::*;
pub use state::*;
use crate::lv::app::socket::Socket;
use crate::lv::vision2::Vision;

pub struct Session {}

pub trait LiveView {
	type Params;
	type Msg;
	type StateKey: Eq + Hash + Clone;

	fn render(state: &Assigns<Self::StateKey>) -> Vision;
	fn mount(params: &Self::Params, session: &Session, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>>;
	fn handle_event(msg: Self::Msg, params: &Self::Params, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>>;
}


