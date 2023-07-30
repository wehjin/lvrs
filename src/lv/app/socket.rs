use std::hash::Hash;
use crate::lv::{Assigns, Value};

#[derive(Debug, Clone)]
pub struct Socket<K: Eq + Hash + Clone> {
	assigns: Assigns<K>,
}

impl<K: Eq + Hash + Clone> Socket<K> {
	pub fn new() -> Self {
		Socket { assigns: Assigns::new() }
	}
	pub fn assigns(&self) -> &Assigns<K> {
		&self.assigns
	}
	pub fn assign(&self, key: K, value: Value) -> Self {
		let mut assigns = self.assigns.clone();
		assigns.assign(key, value);
		Socket { assigns }
	}
	pub fn read(&self, key: &K) -> &Value {
		self.assigns.read(key)
	}
	pub fn read_count(&self, key: &K) -> usize {
		self.assigns.read_event_count(key)
	}
	pub fn update(&self, key: K, f: impl Fn(&Value) -> Value) -> Self {
		let mut assigns = self.assigns.clone();
		assigns.update(key, f);
		Socket { assigns }
	}
}
