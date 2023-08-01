use std::error::Error;
use actix::Addr;
use crate::lv::live::agent;
use crate::lv::live::agent::{LiveAgent, ToHtmlString};
use crate::lv::LiveView;
use crate::lv::server::assets;

#[derive(Debug)]
pub struct LiveApp<T: LiveView + 'static> {
	addr: Addr<LiveAgent<T>>,
}

impl<T: LiveView + 'static> LiveApp<T> {
	pub fn start(params: T::Params) -> Self {
		let addr = agent::start(params);
		LiveApp { addr }
	}

	pub(crate) fn as_agent(&self) -> &Addr<LiveAgent<T>> { &self.addr }

	pub async fn html_string(&self) -> Result<String, Box<dyn Error>> {
		let vision_slice = self.addr.send(ToHtmlString).await?;
		let prefix = assets::prefix();
		let postfix = assets::postfix();
		Ok(format!("{}{}{}", prefix, vision_slice, postfix))
	}
}
