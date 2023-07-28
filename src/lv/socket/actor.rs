use actix::prelude::*;
use crate::lv::{Vision};
use crate::lv::socket::message::HtmlAsString;

pub(crate) struct SocketActor {}

impl Actor for SocketActor {
	type Context = Context<Self>;
}

impl Handler<HtmlAsString> for SocketActor {
	type Result = String;

	fn handle(&mut self, _msg: HtmlAsString, _ctx: &mut Self::Context) -> Self::Result {
		Vision::Live.to_string()
	}
}

pub(crate) fn start() -> Addr<SocketActor> {
	let actor = SocketActor {};
	actor.start()
}
