use serde_json::Value as JsonValue;
use Vision::LiveText;
use crate::lv::Vision::LiveEmoji;

pub enum Vision {
	LiveEmoji,
	LiveText,
}

impl Vision {
	pub fn phx_diffs(&self, later: &Vision) -> JsonValue {
		match (self, later) {
			(&LiveEmoji, LiveEmoji) => json!({"diff":{}}),
			(&LiveEmoji, LiveText) => json!({"diff":{"0":"Hello","2":"Use Emoji"}}),
			(&LiveText, LiveEmoji) => json!({"diff":{"0":"👋","2":"Use Text"}}),
			(&LiveText, LiveText) => json!({"diff":{}}),
		}
	}
	pub fn phx_rendered(&self) -> JsonValue {
		json!({
			"rendered":{
				"0":"👋",
				"1":"Liveviewjstest",
				"2":"Use Text",
				"s":["\n      <div class=\"flex flex-col items-center space-y-10 pt-10\">\n        <div class=\"flex flex-col items-center space-y-5\">\n          <h1 class=\"text-2xl font-bold\">"," ","</h1>\n          <button class=\"bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded\" phx-click=\"toggle\">\n            ","\n          </button>\n        </div>\n        <div class=\"text-center max-w-[200px]\">\n          More documentation and examples at\n          <a class=\"text-blue-500\" href=\"https://liveviewjs.com\" target=\"_blank\" rel=\"noopener noreferrer\"\n            >LiveViewJS.com</a\n          >\n        </div>\n      </div>\n    "]
			}
		})
	}
}

impl ToString for Vision {
	fn to_string(&self) -> String {
		match self {
			LiveEmoji => include_str!("server/assets/hello_live.html").to_string(),
			LiveText => include_str!("server/assets/hello_live_text.html").to_string()
		}
	}
}