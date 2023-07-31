use crate::lv::vision::parts::{Block, Element, Node, NodeList, StaticsBuilder};
use serde_json::{Map, Value as JsonValue};
use crate::lv::vision::slice::Slice;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vision {
	pub roots: Vec<Node>,
	pub fills: Vec<String>,
}

impl Vision {
	pub fn to_html_string(&self) -> String { self.nodes_to_string() }
	pub fn to_phx_rendered(&self) -> JsonValue {
		let mut builder = StaticsBuilder::new();
		self.add_nodes_to_statics(&mut builder);
		let s_value = builder.close();
		{
			let mut m = Map::new();
			for (i, s) in self.fills.iter().enumerate() {
				let v = JsonValue::String(s.to_string());
				m.insert(i.to_string(), v);
			}
			m.insert("s".to_string(), s_value);
			JsonValue::Object(m)
		}
	}
	pub fn to_phx_diff(&self, later_vision: &Vision) -> JsonValue {
		let mut map = Map::new();
		let early_blocks = self.to_nodelist_blocks();
		let early_late_blocks = early_blocks.into_iter().zip(later_vision.to_nodelist_blocks().into_iter());
		for (i, (early, late)) in early_late_blocks.enumerate() {
			if early.value != late.value {
				let key = i.to_string();
				let value = JsonValue::String(late.to_string());
				map.insert(key, value);
			}
		}
		JsonValue::Object(map)
	}
	pub fn new() -> Self { Vision { roots: Vec::new(), fills: Vec::new() } }
	pub fn add_fill(&mut self, value: String) {
		self.fills.push(value);
	}
}

impl NodeList for Vision {
	fn nodes(&self) -> &Vec<Node> { &self.roots }
	fn add_node(&mut self, node: Node) { self.roots.push(node); }
}

impl From<Vec<Slice>> for Vision {
	fn from(slices: Vec<Slice>) -> Self {
		let mut vision = Vision::new();
		let mut open_element: Option<Element> = None;
		let mut parents: Vec<Element> = Vec::new();
		let mut insert_text_before_block = false;
		for slice in slices {
			match slice {
				Slice::OpenElement(name) => {
					if let Some(parent) = open_element.take() {
						parents.push(parent);
					}
					open_element = Some(Element::new(name));
					insert_text_before_block = false;
				}
				Slice::CloseElement(_name) => {
					if let Some(child) = open_element.take() {
						if let Some(mut parent) = parents.pop() {
							parent.add_element(child);
							open_element = Some(parent);
						} else {
							vision.add_element(child);
							open_element = None;
						}
						insert_text_before_block = false;
					} else {
						panic!("exit requires previous entrance")
					}
				}
				Slice::AddAttribute(name, value) => {
					if let Some(mut parent) = open_element.take() {
						parent.add_attribute(name, value);
						open_element = Some(parent);
					} else {
						panic!("attribute requires previous entrance")
					}
				}
				Slice::AddBlock(value) => {
					vision.add_fill(value.to_string());
					if let Some(mut parent) = open_element.take() {
						if insert_text_before_block {
							parent.add_text(" ".into());
						}
						parent.add_block(Block { value });
						open_element = Some(parent);
					} else {
						if insert_text_before_block {
							vision.add_text(" ".into());
						}
						vision.add_block(Block { value });
					}
					insert_text_before_block = true;
				}
				Slice::AddText(text) => {
					if let Some(mut parent) = open_element.take() {
						parent.add_text(text);
						open_element = Some(parent);
					} else {
						vision.add_text(text);
					}
					insert_text_before_block = false;
				}
			}
		}
		vision
	}
}
