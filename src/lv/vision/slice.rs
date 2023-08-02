#[derive(Debug)]
pub(crate) enum Slice {
	OpenElement(String),
	AddAttributeName(String),
	AddAttributeNameExtension(String),
	AddAttributeText(String),
	AddAttributeBlock(String),
	AddEOAOpen,
	AddEOAClose,
	AddBlock(String),
	AddText(String),
	CloseElement(String),
	AddDirective(String),
}

#[cfg(test)]
mod tests {
	#[test]
	fn build() {}
}
