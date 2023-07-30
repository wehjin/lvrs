use serde_json::Value as JsonValue;
use Vision::LiveText;
use crate::lv::vision2::Vision2;
use crate::lv::Vision::LiveEmoji;

pub enum Vision {
	LiveEmoji(Vision2),
	LiveText(Vision2),
}

impl Vision {
	fn vision2(&self) -> &Vision2 {
		match self {
			LiveEmoji(v2) => &v2,
			LiveText(v2) => &v2,
		}
	}
	pub fn phx_diffs(&self, later: &Vision) -> JsonValue {
		match (self, later) {
			(&LiveEmoji(_), LiveEmoji(_)) => json!({"diff":{}}),
			(&LiveEmoji(_), LiveText(_)) => json!({"diff":{"0":"Hello","2":"Use Emoji"}}),
			(&LiveText(_), LiveEmoji(_)) => json!({"diff":{"0":"👋","2":"Use Text"}}),
			(&LiveText(_), LiveText(_)) => json!({"diff":{}}),
		}
	}
	pub fn phx_rendered(&self) -> JsonValue {
		json!({"rendered": self.vision2().to_phx_rendered()})
	}

	pub fn to_html_string(&self) -> String {
		self.vision2().to_html_string()
	}
}