use serde_json::Value as JsonValue;
use crate::lv::phx::PhxMsgType::{Event, Heartbeat, Join};

#[derive(Debug, Clone)]
pub enum PhxMsgType {
	Join,
	Heartbeat,
	Event,
	Unknown(String),
}

impl From<&JsonValue> for PhxMsgType {
	fn from(value: &JsonValue) -> Self {
		let s = value.as_str().unwrap();
		match s {
			"phx_join" => Join,
			"heartbeat" => Heartbeat,
			"event" => Event,
			&_ => PhxMsgType::Unknown(s.to_string())
		}
	}
}

pub const PHX_REPLY: &str = "phx_reply";

pub fn reply_ok(response: JsonValue) -> JsonValue {
	json!({
		"status":"ok",
		"response":response
	})
}
