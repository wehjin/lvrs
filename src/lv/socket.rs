use std::collections::HashMap;
use std::hash::Hash;
use crate::lv::Value;

pub struct Socket<K: Eq + Hash> {
	state: HashMap<K, Value>,
}

impl<K: Eq + Hash> Socket<K> {
	pub fn assign(self, key: K, value: Value) -> Self {
		let mut state = self.state;
		state.insert(key, value);
		Socket { state }
	}
}

pub(crate) mod actor {
	use actix::prelude::*;
	use crate::lv::Vision;

	#[derive(Message)]
	#[rtype(result = "String")]
	pub(crate) struct HtmlAsString;

	pub(crate) struct SocketActor {}

	impl Actor for SocketActor {
		type Context = Context<Self>;
	}

	impl Handler<HtmlAsString> for SocketActor {
		type Result = String;

		fn handle(&mut self, msg: HtmlAsString, ctx: &mut Self::Context) -> Self::Result {
			Vision::Live.to_string()
		}
	}

	pub(crate) fn start() -> Addr<SocketActor> {
		let actor = SocketActor {};
		let addr = actor.start();
		addr
	}
}
