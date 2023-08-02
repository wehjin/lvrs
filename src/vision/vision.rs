use crate::vision::parts::{AttributeValue, Block, Element, Node, NodeList, StaticsBuilder};
use serde_json::{Map, Value as JsonValue};
use crate::vision::slice::Slice;

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
		struct Cursor {
			pub vision: Vision,
			pub parents: Vec<Element>,
			pub insert_text_before_block: bool,
			pub active_element: Option<Element>,
			pub active_attr_name: Vec<String>,
		}
		impl Cursor {
			pub fn start_open_element(&mut self, name: String) {
				if let Some(element) = self.active_element.take() {
					self.parents.push(element);
				}
				self.active_element = Some(Element::new(name));
				self.active_attr_name.clear();
			}
			pub fn finish_previous_attr(&mut self, value: AttributeValue) {
				if self.active_attr_name.len() > 0 {
					if let Some(mut element) = self.active_element.take() {
						element.add_attribute(self.active_attr_name.join("-"), value);
						self.active_element = Some(element);
					} else {
						panic!("finishing an attribute requires active element")
					}
				}
				self.active_attr_name.clear();
			}
			pub fn start_attr_name(&mut self, name: String) {
				self.finish_previous_attr(AttributeValue::None);
				self.active_attr_name = vec![name];
			}
			pub fn extend_attr_name(&mut self, extension: String) {
				self.active_attr_name.push(extension);
			}
			pub fn end_attr_with_value(&mut self, value: AttributeValue) {
				self.finish_previous_attr(value);
				self.active_attr_name.clear();
			}
			pub fn finish_open_element(&mut self, and_close_element: bool) {
				self.finish_previous_attr(AttributeValue::None);
				self.insert_text_before_block = false;
				if and_close_element {
					self.close_element();
				}
			}
			pub fn add_text(&mut self, text: String) {
				if let Some(mut element) = self.active_element.take() {
					element.add_text(text);
					self.active_element = Some(element);
				} else {
					self.vision.add_text(text);
				}
				self.insert_text_before_block = false;
			}
			pub fn add_block(&mut self, value: String) {
				self.vision.add_fill(value.to_string());
				if let Some(mut element) = self.active_element.take() {
					if self.insert_text_before_block {
						element.add_text(" ".into());
					}
					element.add_block(Block { value });
					self.active_element = Some(element);
				} else {
					if self.insert_text_before_block {
						self.vision.add_text(" ".into());
					}
					self.vision.add_block(Block { value });
				}
				self.insert_text_before_block = true;
			}
			pub fn add_directive(&mut self, text: String) {
				if let Some(mut element) = self.active_element.take() {
					element.add_directive(text);
					self.active_element = Some(element);
				} else {
					self.vision.add_directive(text);
				}
				self.insert_text_before_block = false;
			}
			pub fn close_element(&mut self) {
				if let Some(element) = self.active_element.take() {
					if let Some(mut parent) = self.parents.pop() {
						parent.add_element(element);
						self.active_element = Some(parent);
					} else {
						self.vision.add_element(element);
						self.active_element = None;
					}
					self.insert_text_before_block = false;
				} else {
					panic!("exit requires previous entrance")
				}
			}
		}
		let mut cursor = Cursor {
			vision: Vision::new(),
			parents: Vec::new(),
			insert_text_before_block: false,
			active_element: None,
			active_attr_name: Vec::new(),
		};
		for slice in slices {
			match slice {
				Slice::OpenElement(name) => {
					cursor.start_open_element(name);
				}
				Slice::AddAttributeName(name) => {
					cursor.start_attr_name(name);
				}
				Slice::AddAttributeNameExtension(extension) => {
					cursor.extend_attr_name(extension);
				}
				Slice::AddAttributeBlock(block) => {
					cursor.end_attr_with_value(AttributeValue::Block(block))
				}
				Slice::AddAttributeText(string) => {
					cursor.end_attr_with_value(AttributeValue::String(string))
				}
				Slice::AddEOAOpen => {
					cursor.finish_open_element(false);
				}
				Slice::AddEOAClose => {
					cursor.finish_open_element(true);
				}
				Slice::AddText(text) => {
					cursor.add_text(text);
				}
				Slice::CloseElement(_name) => {
					cursor.close_element();
				}
				Slice::AddBlock(value) => {
					cursor.add_block(value);
				}
				Slice::AddDirective(text) => {
					cursor.add_directive(text);
				}
			}
		}
		cursor.vision
	}
}
