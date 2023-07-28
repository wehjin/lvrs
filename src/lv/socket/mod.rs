use std::collections::HashMap;
use std::hash::Hash;
use crate::lv::Value;

pub struct Socket<K: Eq + Hash> {
	_assigns: HashMap<K, Value>,
}

pub(crate) mod message;
pub(crate) mod actor;
