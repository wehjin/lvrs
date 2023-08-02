use serde_json::Value as JsonValue;

pub trait NodeList {
	fn nodes(&self) -> &Vec<Node>;
	fn to_nodelist_blocks(&self) -> Vec<&Block> {
		let mut blocks = Vec::new();
		self.get_nodelist_blocks(&mut blocks);
		blocks
	}
	fn get_nodelist_blocks<'a>(&'a self, blocks: &mut Vec<&'a Block>) {
		for node in self.nodes() {
			node.get_blocks(blocks);
		}
	}
	fn add_node(&mut self, node: Node);
	fn add_element(&mut self, element: Element) {
		let node = Node::Element(element);
		self.add_node(node);
	}
	fn add_block(&mut self, block: Block) {
		let node = Node::Block(block);
		self.add_node(node);
	}
	fn add_text(&mut self, string: String) {
		let node = Node::Text(string);
		self.add_node(node);
	}
	fn add_directive(&mut self, string: String) {
		let node = Node::Directive(string);
		self.add_node(node)
	}
	fn nodes_to_string(&self) -> String {
		let mut string = String::new();
		for node in self.nodes() {
			let more = format!("{}", node.to_string());
			string.push_str(&more);
		}
		string
	}
	fn add_nodes_to_statics(&self, builder: &mut StaticsBuilder) {
		for node in self.nodes() {
			node.add_to_statics(builder)
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Element {
	name: String,
	attributes: Vec<Attribute>,
	child_nodes: Vec<Node>,
}

impl Element {
	pub fn new(name: String) -> Self {
		Element { name, attributes: Vec::new(), child_nodes: Vec::new() }
	}
	pub fn add_attribute(&mut self, name: String, value: AttributeValue) {
		self.attributes.push(Attribute { name, value })
	}
	fn to_attributes_string(&self) -> String {
		let mut attrs_string = String::new();
		for attr in &self.attributes {
			let more = format!(" {}", attr.to_string());
			attrs_string.push_str(&more);
		}
		attrs_string.trim().to_string()
	}
	fn to_entrance_string(&self) -> String {
		let attrs_string = self.to_attributes_string();
		if attrs_string.is_empty() {
			format!("<{}>", &self.name)
		} else {
			format!("<{} {}>", &self.name, attrs_string)
		}
	}
	fn to_exit_string(&self) -> String { format!("</{}>", &self.name) }
	fn to_meta_string(&self) -> String {
		let attrs_string = self.to_attributes_string();
		format!("<meta {} />\n", attrs_string)
	}

	fn add_to_statics(&self, builder: &mut StaticsBuilder) {
		builder.add_static(&self.to_entrance_string());
		self.add_nodes_to_statics(builder);
		builder.add_static(&self.to_exit_string());
	}
}

impl ToString for Element {
	fn to_string(&self) -> String {
		if self.name == "meta" {
			self.to_meta_string()
		} else {
			let gap = if self.child_nodes.len() < 2 {
				""
			} else {
				"\n"
			};
			format!("{}{}{}{}{}\n", self.to_entrance_string(), &gap, self.nodes_to_string(), &gap, self.to_exit_string())
		}
	}
}

impl NodeList for Element {
	fn nodes(&self) -> &Vec<Node> { &self.child_nodes }
	fn add_node(&mut self, node: Node) {
		self.child_nodes.push(node);
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeValue {
	Block(String),
	String(String),
	None,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
	pub name: String,
	pub value: AttributeValue,
}

impl ToString for Attribute {
	fn to_string(&self) -> String {
		let value = match &self.value {
			AttributeValue::Block(value) => Some(value),
			AttributeValue::String(string) => Some(string),
			AttributeValue::None => None,
		};
		if let Some(value) = value {
			format!("{}=\"{}\"", &self.name, value)
		} else {
			format!("{}", &self.name)
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
	pub value: String,
}

impl ToString for Block {
	fn to_string(&self) -> String {
		self.value.to_string()
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
	Element(Element),
	Block(Block),
	Text(String),
	Directive(String),
}

impl Node {
	pub(crate) fn add_to_statics(&self, builder: &mut StaticsBuilder) {
		match self {
			Node::Element(el) => el.add_to_statics(builder),
			Node::Block(_) => builder.add_dynamic(),
			Node::Text(s) => builder.add_static(s),
			Node::Directive(_) => builder.add_static(&self.to_string()),
		}
	}
	pub(crate) fn get_blocks<'a>(&'a self, blocks: &mut Vec<&'a Block>) {
		match self {
			Node::Element(el) => el.get_nodelist_blocks(blocks),
			Node::Block(b) => blocks.push(b),
			Node::Text(_) => (),
			Node::Directive(_) => (),
		}
	}
}

impl ToString for Node {
	fn to_string(&self) -> String {
		match self {
			Node::Element(el) => el.to_string(),
			Node::Block(b) => b.to_string(),
			Node::Text(t) => t.to_string(),
			Node::Directive(s) => format!("<!{}>\n", s),
		}
	}
}


pub struct StaticsBuilder {
	accumulated: Option<String>,
	closed: Vec<String>,
}

impl StaticsBuilder {
	pub fn new() -> Self {
		StaticsBuilder { accumulated: None, closed: Vec::new() }
	}
	pub fn add_static(&mut self, text: &str) {
		if let Some(mut accumulated) = self.accumulated.take() {
			accumulated.push_str(text);
			self.accumulated = Some(accumulated);
		} else {
			self.accumulated = Some(text.to_string());
		}
	}
	pub fn add_dynamic(&mut self) {
		if let Some(accumulated) = self.accumulated.take() {
			self.closed.push(accumulated);
		} else {
			self.closed.push("".to_string())
		}
	}
	pub fn close(&mut self) -> JsonValue {
		if let Some(accumulated) = self.accumulated.take() {
			self.closed.push(accumulated);
		} else {
			self.closed.push("".to_string());
		}
		let vec = self.closed.iter().map(|it| JsonValue::String(it.to_owned())).collect::<Vec<_>>();
		JsonValue::Array(vec)
	}
}
