use std::error::Error;
use actix::Addr;
use crate::lv::app::agent;
use crate::lv::app::agent::{AppAgent, ToHtmlString};
use crate::lv::server::assets;

#[derive(Debug)]
pub(crate) struct AppState {
	sample_app: Addr<AppAgent>,
}

impl AppState {
	pub fn new() -> Self {
		AppState { sample_app: agent::start() }
	}
	pub fn app_worker(&self) -> &Addr<AppAgent> { &self.sample_app }
	pub async fn html_string(&self) -> Result<String, Box<dyn Error>> {
		let vision_slice = self.sample_app.send(ToHtmlString).await?;
		let prefix = assets::prefix();
		let postfix = assets::postfix();
		Ok(format!("{}{}{}", prefix, vision_slice, postfix))
	}
}
