use std::error::Error;
use actix::Addr;
use crate::lv::server::assets;
use crate::lv::socket;
use crate::lv::socket::actor::SocketActor;
use crate::lv::socket::message::HtmlAsString;

#[derive(Debug)]
pub(crate) struct LiveApp {
	socket: Addr<SocketActor>,
}

impl LiveApp {
	pub fn new() -> Self {
		let socket = socket::actor::start();
		LiveApp { socket }
	}
	pub async fn html_string(&self) -> Result<String, Box<dyn Error>> {
		let live_slice = self.socket.send(HtmlAsString).await?;
		let prefix = assets::prefix();
		let postfix = assets::postfix();
		Ok(format!("{}{}{}", prefix, live_slice, postfix))
	}
}
