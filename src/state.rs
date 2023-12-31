use std::collections::HashMap;
use std::hash::Hash;
use crate::Value;

#[derive(Debug)]
pub struct Assigns<K: Eq + Hash> {
	assigns: HashMap<K, (Value, usize)>,
}

impl<K: Eq + Hash> Assigns<K> {
	pub fn new() -> Self {
		Assigns { assigns: HashMap::new() }
	}
	pub fn assign(&mut self, key: K, value: Value) -> &mut Self {
		let count = self.read_event_count(&key);
		self.assigns.insert(key, (value, count + 1));
		self
	}
	pub fn update(&mut self, key: K, f: impl Fn(&Value) -> Value) -> &mut Self {
		let (value, count) = self.assigns.get(&key).expect("update value must exist");
		let new_value = f(value);
		self.assigns.insert(key, (new_value, count + 1));
		self
	}
	pub fn read(&self, key: &K) -> &Value {
		let existing = self.assigns.get(key).expect("read value must exist");
		&existing.0
	}
	pub fn read_event_count(&self, key: &K) -> usize {
		let count = self.assigns.get(&&key).map(|it| it.1).unwrap_or(0);
		count
	}

	pub fn event_count(&self) -> usize {
		self.assigns.values().fold(0, |count, next| count + next.1)
	}
}
