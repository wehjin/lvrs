pub enum Vision {
	Live,
}

impl ToString for Vision {
	fn to_string(&self) -> String {
		match self {
			Vision::Live => include_str!("app/assets/hello_live.html").to_string(),
		}
	}
}