use std::error::Error;
use crate::sample::SampleAppAssignKey::UsingEmoji;
use crate::lv::{LiveView, Session, Assigns, Value, FakeVision};
use crate::lv::app::socket::Socket;
use crate::lv::vision2::{vision};
use crate::lv::vision2::slice::{Slice, SliceListBuilder};

pub struct SampleAppParams {}

pub enum SampleAppMsg { Toggle }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SampleAppAssignKey { UsingEmoji }

pub struct SampleApp {}

impl LiveView for SampleApp {
	type Params = SampleAppParams;
	type Msg = SampleAppMsg;
	type StateKey = SampleAppAssignKey;

	fn render(assigns: &Assigns<Self::StateKey>) -> FakeVision {
		let using_emoji = assigns.read(&UsingEmoji).to_bool();
		let fills = match using_emoji {
			true => emoji_fills(),
			false => text_fills(),
		};
		let slices = build_slices(fills);
		FakeVision { vision2: vision(slices) }
	}

	fn mount(_params: &Self::Params, _session: &Session, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		let socket = socket.assign(UsingEmoji, Value::Bool(true));
		Ok(socket)
	}

	fn handle_event(msg: Self::Msg, _params: &Self::Params, socket: &Socket<Self::StateKey>) -> Result<Socket<Self::StateKey>, Box<dyn Error>> {
		match msg {
			SampleAppMsg::Toggle => {
				let socket = socket.update(UsingEmoji, Value::flip);
				Ok(socket)
			}
		}
	}
}

fn build_slices(fills: Vec<String>) -> Vec<Slice> {
	let mut builder = SliceListBuilder::new();
	builder.add_open("div".to_string(), vec![
		("class".to_string(), "flex flex-col items-center space-y-10 pt-10".to_string())
	]);
	builder.add_open("div".to_string(), vec![
		("class".to_string(), "flex flex-col items-center space-y-5".to_string())
	]);
	builder.add_open("h1".to_string(), vec![
		("class".to_string(), "text-2xl font-bold".to_string())
	]);
	builder.add_block(fills[0].to_string());
	builder.add_text(" ".to_string());
	builder.add_block(fills[1].to_string());
	builder.add_close("h1".to_string());
	builder.add_open("button".to_string(), vec![
		("class".to_string(), "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded".to_string()),
		("phx-click".to_string(), "toggle".to_string()),
	]);
	builder.add_block(fills[2].to_string());
	builder.add_close("button".to_string());
	builder.add_close("div".to_string());
	builder.add_open("div".to_string(), vec![
		("class".to_string(), "text-center max-w-[200px]".to_string())
	]);
	builder.add_text("\n          More documentation and examples at\n          ".to_string());
	builder.add_open("a".to_string(), vec![
		("class".to_string(), "text-blue-500".to_string()),
		("href".to_string(), "https://liveviewjs.com".to_string()),
		("target".to_string(), "_blank".to_string()),
		("rel".to_string(), "noopener noreferrer".to_string()),
	]);
	builder.add_text("LiveViewJS.com".to_string());
	builder.add_close("a".to_string());
	builder.add_close("div".to_string());
	builder.add_close("div".to_string());
	builder.build()
}

fn emoji_fills() -> Vec<String> {
	vec![
		"👋".to_string(),
		"Liveviewjstest".to_string(),
		"Use Text".to_string(),
	]
}

fn text_fills() -> Vec<String> {
	vec![
		"Hello".to_string(),
		"Liveviewjstest".to_string(),
		"Use Emoji".to_string(),
	]
}

#[cfg(test)]
mod tests {
	use crate::sample::{build_slices, emoji_fills};

	#[test]
	fn test0() {
		let vision = vision2(build_slices(emoji_fills()));
		let rendered = vision.to_phx_rendered();
		let rendered = serde_json::to_string_pretty(&rendered).unwrap();
		println!("{}", rendered)
	}
}
