use lv::prelude::*;
use crate::lv;
use crate::lv::vision2::Vision2;

// TODO Eliminate FakeVision
pub struct FakeVision {
	pub vision2: Vision2,
}

impl FakeVision {
	pub fn to_html_string(&self) -> String {
		self.vision2.to_html_string()
	}

	pub fn phx_rendered(&self) -> JsonValue {
		json!({
			"rendered": self.vision2.to_phx_reply_rendered()
		})
	}

	pub fn phx_diff(&self, late_vision: &FakeVision) -> JsonValue {
		json!({
			"diff": self.vision2.to_phx_reply_diff(&late_vision.vision2)
		})
	}
}