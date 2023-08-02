use std::hash::Hash;
use crate::{Assigns, Value};

#[derive(Debug)]
pub struct Socket<K: Eq + Hash> {
	assigns: Assigns<K>,
}

impl<K: Eq + Hash> Socket<K> {
	pub fn new() -> Self {
		Socket { assigns: Assigns::new() }
	}
	pub fn assigns(&self) -> &Assigns<K> {
		&self.assigns
	}
	pub fn read(&self, key: &K) -> &Value {
		self.assigns.read(key)
	}
	pub fn read_count(&self, key: &K) -> usize {
		self.assigns.read_event_count(key)
	}
	pub fn assign(&mut self, key: K, value: Value) -> &mut Self {
		self.assigns.assign(key, value);
		self
	}
	pub fn update(&mut self, key: K, f: impl Fn(&Value) -> Value) -> &mut Self {
		self.assigns.update(key, f);
		self
	}
}
