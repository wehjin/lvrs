use std::error::Error;
use crate::sample::SampleAppAssignKeys::UsingEmoji;
use crate::lv::{LiveView, Session, Assigns, Value};
use crate::lv::app::socket::Socket;
use crate::lv::vision::{vision, Vision};
use crate::lv::vision::slice::{Slice, SliceListBuilder};

pub struct SampleAppParams {}

pub enum SampleAppMsg { Toggle }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SampleAppAssignKeys { UsingEmoji }

pub struct SampleApp {}

impl LiveView for SampleApp {
	type Params = SampleAppParams;
	type Msg = SampleAppMsg;
	type AssignKeys = SampleAppAssignKeys;

	fn mount(_params: &Self::Params, _session: &Session, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		let socket = socket.assign(UsingEmoji, Value::Bool(true));
		Ok(socket)
	}

	fn handle_event(msg: Self::Msg, _params: &Self::Params, socket: &Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		match msg {
			SampleAppMsg::Toggle => {
				let socket = socket.update(UsingEmoji, Value::flip);
				Ok(socket)
			}
		}
	}

	fn render(assigns: &Assigns<Self::AssignKeys>) -> Vision {
		let using_emoji = assigns.read(&UsingEmoji).to_bool();
		let fills = match using_emoji {
			true => emoji_fills(),
			false => text_fills(),
		};
		let slices = build_slices(fills);
		vision(slices)
	}
}

fn build_slices(fills: Vec<String>) -> Vec<Slice> {
	let mut builder = SliceListBuilder::new();
	builder.add_open("div", vec![("class", "flex flex-col items-center space-y-10 pt-10")]);
	builder.add_open("div", vec![("class", "flex flex-col items-center space-y-5")]);
	builder.add_open("h1", vec![("class", "text-2xl font-bold")]);
	builder.add_block(fills[0].to_string());
	builder.add_text(" ");
	builder.add_block(fills[1].to_string());
	builder.add_close("h1");
	builder.add_open("button", vec![
		("class", "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"),
		("phx-click", "toggle"),
	]);
	builder.add_block(fills[2].to_string());
	builder.add_close("button");
	builder.add_close("div");
	builder.add_open("div", vec![("class", "text-center max-w-[200px]")]);
	builder.add_text("\n          More documentation and examples at\n          ");
	builder.add_open("a", vec![
		("class", "text-blue-500"),
		("href", "https://liveviewjs.com"),
		("target", "_blank"),
		("rel", "noopener noreferrer"),
	]);
	builder.add_text("LiveViewJS.com");
	builder.add_close("a");
	builder.add_close("div");
	builder.add_close("div");
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
