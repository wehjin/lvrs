pub(crate) enum Slice {
	OpenElement(String),
	CloseElement(String),
	AddAttribute(String, String),
	AddBlock(String),
	AddText(String),
}

pub(crate) struct SliceListBuilder {
	slices: Vec<Slice>,
}

impl SliceListBuilder {
	pub fn add_open(&mut self, name: &str, attrs: Vec<(&str, &str)>) -> &mut Self {
		self.slices.push(Slice::OpenElement(name.to_string()));
		for (name, value) in attrs {
			self.slices.push(Slice::AddAttribute(name.to_string(), value.to_string()))
		}
		self
	}
	pub fn add_block(&mut self, block_text: String) -> &mut Self {
		self.slices.push(Slice::AddBlock(block_text));
		self
	}
	pub fn add_text(&mut self, text: &str) -> &mut Self {
		self.slices.push(Slice::AddText(text.to_string()));
		self
	}
	pub fn add_close(&mut self, name: &str) -> &mut Self {
		self.slices.push(Slice::CloseElement(name.to_string()));
		self
	}
	pub fn build(self) -> Vec<Slice> { self.slices }
	pub fn new() -> Self {
		SliceListBuilder { slices: Vec::new() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn build() {
		let mut builder = SliceListBuilder::new();
	}
}
