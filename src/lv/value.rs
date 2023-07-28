pub enum Value {
	Bool(bool),
	Size(usize),
}

impl From<bool> for Value {
	fn from(value: bool) -> Self { Value::Bool(value) }
}

impl Value {
	pub fn to_bool(&self) -> bool {
		match self {
			Value::Bool(v) => *v,
			_ => false
		}
	}
}
