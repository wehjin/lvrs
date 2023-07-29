mod slice;
mod parts;

use crate::lv::vision2::parts::{Block, Element, Node, NodeList, StaticsBuilder};
use crate::lv::vision2::slice::Slice;
use serde_json::{Map, Value as JsonValue};


pub struct Vision2 {
	roots: Vec<Node>,
}

impl Vision2 {
	pub fn new() -> Self { Vision2 { roots: Vec::new() } }
	pub fn to_string(&self) -> String { self.nodes_to_string() }
	pub fn to_phx_rendered(&self) -> JsonValue {
		let mut builder = StaticsBuilder::new();
		self.add_nodes_to_statics(&mut builder);
		let s_value = builder.close();
		{
			let mut m = Map::new();
			let fills = vec!["👋".to_string(), "Liveviewjstest".to_string(), "Use Text".to_string()];
			for (i, s) in fills.iter().enumerate() {
				let v = JsonValue::String(s.to_string());
				m.insert(i.to_string(), v);
			}
			m.insert("s".to_string(), s_value);
			JsonValue::Object(m)
		}
	}
}

impl NodeList for Vision2 {
	fn nodes(&self) -> &Vec<Node> { &self.roots }
	fn add_node(&mut self, node: Node) {
		self.roots.push(node);
	}
}

pub fn vision() -> Vision2 {
	let mut vision = Vision2::new();
	let mut open_element: Option<Element> = None;
	let mut parents: Vec<Element> = Vec::new();
	for slice in slice::slices() {
		match slice {
			Slice::OpenElement(name) => {
				if let Some(parent) = open_element.take() {
					parents.push(parent);
				}
				open_element = Some(Element::new(name));
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
				} else {
					panic!("exit requires previous entrance")
				}
			}
			Slice::AddAttribute(name, value) => {
				if let Some(mut child) = open_element.take() {
					child.add_attribute(name, value);
					open_element = Some(child);
				} else {
					panic!("attribute requires previous entrance")
				}
			}
			Slice::AddBlock(index) => {
				if let Some(mut parent) = open_element.take() {
					parent.add_block(Block { index });
					open_element = Some(parent);
				} else {
					vision.add_block(Block { index });
				}
			}
			Slice::AddText(text) => {
				if let Some(mut parent) = open_element.take() {
					parent.add_text(text);
					open_element = Some(parent);
				} else {
					vision.add_text(text);
				}
			}
		}
	}
	vision
}

#[cfg(test)]
mod tests {
	use crate::lv::vision2::vision;

	#[test]
	fn test0() {
		let vision = vision();
		let rendered = vision.to_phx_rendered();
		let rendered = serde_json::to_string_pretty(&rendered).unwrap();
		println!("{}", rendered)
	}
}


