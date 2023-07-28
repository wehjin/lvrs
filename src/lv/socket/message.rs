use std::hash::Hash;
use actix::Message;
use crate::lv::Value;

#[derive(Message)]
#[rtype(result = "Value")]
pub(crate) struct AssignedValue<K: Eq + Hash> (K);

impl<K: Eq + Hash> AssignedValue<K> {
	pub fn key(&self) -> &K { &self.0 }
}

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct HtmlAsString;
