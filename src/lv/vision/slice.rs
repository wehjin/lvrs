#[derive(Debug)]
pub(crate) enum Slice {
	OpenElement(String),
	CloseElement(String),
	AddAttribute(String, String),
	AddBlock(String),
	AddText(String),
	AddDirective(String),
}

#[cfg(test)]
mod tests {
	#[test]
	fn build() {}
}
