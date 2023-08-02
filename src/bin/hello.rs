#[macro_use]
extern crate lv;

use lv::server::{Route};

use std::error::Error;
use lv::{Assigns, LiveMsg, LiveView, Session, Value};
use lv::live::socket::Socket;
use lv::vision::Vision;
use crate::SampleAppAssignKeys::UsingEmoji;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let route = Route::new("/hello");
	lv::server::start::<SampleApp>(SampleAppParams, route).await
}

pub struct SampleAppParams;

pub enum SampleAppMsg { Toggle }

impl LiveMsg for SampleAppMsg {
	type Err = String;

	fn from_str(s: impl AsRef<str>) -> Result<Self, Self::Err> {
		match s.as_ref() {
			"toggle" => Ok(SampleAppMsg::Toggle),
			&_ => Err(format!("no msg for \"{}\"", s.as_ref())),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SampleAppAssignKeys { UsingEmoji }

pub struct SampleApp;

impl LiveView for SampleApp {
	type Params = SampleAppParams;
	type Msg = SampleAppMsg;
	type AssignKeys = SampleAppAssignKeys;

	fn mount(_params: &Self::Params, _session: &Session, socket: Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		let mut socket = socket;
		socket.assign(UsingEmoji, Value::Bool(true));
		Ok(socket)
	}

	fn handle_event(msg: Self::Msg, _params: &Self::Params, socket: Socket<Self::AssignKeys>) -> Result<Socket<Self::AssignKeys>, Box<dyn Error>> {
		let socket = match msg {
			SampleAppMsg::Toggle => {
				let mut socket = socket;
				socket.update(UsingEmoji, Value::flip);
				socket
			}
		};
		Ok(socket)
	}

	fn render(assigns: &Assigns<Self::AssignKeys>) -> Vision {
		let use_emoji = assigns.read(&UsingEmoji).to_bool();
		let hello = match use_emoji {
			true => "👋",
			false => "Hello",
		};
		let name = "Liveviewjstest";
		vision! {
			<div class = "flex flex-col items-center space-y-10 pt-10" >
			<div class ="flex flex-col items-center space-y-5" >
			<h1 class = "text-2xl font-bold" >{hello} {name}< / h1 >
			<button class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" phx-click = "toggle" >
			{if use_emoji { "Use Text"} else {"Use Emoji"}}
			</button >
			</div >
			<div class = "text-center max-w-[200px]" >
			"\n          More documentation and examples at\n          "
			<a class = "text-blue-500" href = "https://liveviewjs.com" target ="_blank" rel = "noopener noreferrer" >
			"LiveViewJS.com"
			</a >
			</div >
			</div >
		}
	}
}

#[cfg(test)]
mod tests {
	use lv::{Assigns, LiveView};
	use super::SampleApp;
	use super::SampleAppAssignKeys::UsingEmoji;

	#[test]
	fn test0() {
		let mut assigns = Assigns::new();
		assigns.assign(UsingEmoji, true.into());
		let vision = SampleApp::render(&assigns);
		let rendered = vision.to_phx_rendered();
		let rendered = serde_json::to_string_pretty(&rendered).unwrap();
		println!("{}", rendered)
	}
}
