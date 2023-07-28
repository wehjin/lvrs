use serde_json::Value;
use crate::lv::phx::PhxMsgType::{Event, Heartbeat, Join};

#[derive(Debug, Clone)]
pub enum PhxMsgType {
	Join,
	Heartbeat,
	Event,
	Unknown(String),
}

impl From<&Value> for PhxMsgType {
	fn from(value: &Value) -> Self {
		let s = value.as_str().unwrap();
		match s {
			"phx_join" => Join,
			"heartbeat" => Heartbeat,
			"event" => Event,
			&_ => PhxMsgType::Unknown(s.to_string())
		}
	}
}
