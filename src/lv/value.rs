#[derive(Debug, Clone)]
pub enum Value {
	Bool(bool),
	Size(usize),
}

impl Value {
	pub fn to_bool(&self) -> bool {
		match self {
			Value::Bool(v) => *v,
			_ => false
		}
	}
	pub fn flip(&self) -> Value {
		match self {
			Value::Bool(b) => Value::Bool(!*b),
			Value::Size(_) => panic!("must be bool")
		}
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self { Value::Bool(value) }
}
