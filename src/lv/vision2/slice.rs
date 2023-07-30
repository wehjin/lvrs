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
	pub fn new() -> Self {
		SliceListBuilder { slices: Vec::new() }
	}
	pub fn add_open(&mut self, name: String, attrs: Vec<(String, String)>) -> &mut Self {
		self.slices.push(Slice::OpenElement(name));
		for (name, value) in attrs {
			self.slices.push(Slice::AddAttribute(name, value))
		}
		self
	}
	pub fn add_block(&mut self, block_text: String) -> &mut Self {
		self.slices.push(Slice::AddBlock(block_text));
		self
	}
	pub fn add_text(&mut self, text: String) -> &mut Self {
		self.slices.push(Slice::AddText(text));
		self
	}
	pub fn add_close(&mut self, name: String) -> &mut Self {
		self.slices.push(Slice::CloseElement(name));
		self
	}
	pub fn build(self) -> Vec<Slice> { self.slices }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn build() {
		let mut builder = SliceListBuilder::new();
	}
}
