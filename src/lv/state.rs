use std::collections::HashMap;
use std::hash::Hash;
use crate::lv::Value;

pub struct State<K: Eq + Hash> {
	values: HashMap<K, Value>,
}

impl<K: Eq + Hash> State<K> {
	pub fn get(&self, key: &K) -> Option<&Value> { self.values.get(key) }
}
