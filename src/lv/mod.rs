use std::error::Error;
use std::hash::Hash;

pub use vision::*;
pub use value::*;
pub use state::*;
use crate::lv::socket::Socket;

pub struct Session {}

pub trait LiveView {
	type Params;
	type Msg;
	type StateKey: Eq + Hash;

	fn render(state: &State<Self::StateKey>) -> Vision;
	fn mount(params: &Self::Params, session: &Session, socket: Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>>;
	fn handle_event(_msg: Self::Msg, _params: &Self::Params, socket: Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		Ok(socket)
	}
}

pub mod server;
pub mod socket;
mod vision;
mod value;
mod state;
mod phx;


