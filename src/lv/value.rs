use std::ops;
use std::ops::Neg;

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
}

impl From<bool> for Value {
	fn from(value: bool) -> Self { Value::Bool(value) }
}

impl ops::Not for Value {
	type Output = Value;

	fn not(self) -> Self::Output {
		let out = match self {
			Value::Bool(b) => Value::Bool(!b),
			Value::Size(_) => panic!("must be bool")
		};
		out
	}
}
