use serde_json::{Map};
use lv::prelude::*;
use crate::lv;

#[derive(Debug)]
pub struct Context {
	pub join_ref: Option<String>,
	pub msg_ref: Option<String>,
	pub topic: String,
}

impl Context {
	pub fn to_phx_reply(&self, response: JsonValue) -> JsonValue {
		json!([
			&self.join_ref,
			&self.msg_ref,
			&self.topic,
			"phx_reply",
			{"status":"ok","response": response},
		])
	}
}

#[derive(Debug)]
pub enum PhxEvent {
	Join(Context, Map<String, JsonValue>),
	Heartbeat(Context),
	UserEvent {
		context: Context,
		category: String,
		event: String,
		value: Map<String, JsonValue>,
	},
	NotImplemented(Context, String, Map<String, JsonValue>),
}

impl PhxEvent {
	pub fn from_array(array: &Vec<JsonValue>) -> Self {
		let context = Context {
			join_ref: option_string_from_json(&array[0]),
			msg_ref: option_string_from_json(&array[1]),
			topic: array[2].as_str().unwrap().to_string(),
		};
		let event = array[3].as_str().unwrap();
		let payload = array[4].as_object().unwrap();
		match event {
			"phx_join" => PhxEvent::Join(context, payload.clone()),
			"heartbeat" => PhxEvent::Heartbeat(context),
			"event" => {
				PhxEvent::UserEvent {
					context,
					category: payload["type"].as_str().unwrap().to_string(),
					event: payload["event"].as_str().unwrap().to_string(),
					value: payload["value"].as_object().unwrap().clone(),
				}
			}
			&_ => PhxEvent::NotImplemented(context, event.to_string(), payload.clone())
		}
	}

	pub fn context(&self) -> &Context {
		match self {
			PhxEvent::Join(context, _) => context,
			PhxEvent::Heartbeat(context) => context,
			PhxEvent::UserEvent { context, .. } => context,
			PhxEvent::NotImplemented(context, _, _) => context,
		}
	}
}

fn option_string_from_json(value: &JsonValue) -> Option<String> {
	match value {
		JsonValue::String(s) => Some(s.to_string()),
		_ => None,
	}
}
