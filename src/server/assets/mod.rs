#[cfg(test)]
pub(crate) mod tests {
	pub fn prefix() -> String {
		include_str!("hello_prefix.html").to_string()
	}

	pub fn postfix() -> String {
		include_str!("hello_postfix.html").to_string()
	}
}
