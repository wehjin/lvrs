#[macro_use]
extern crate serde_json;

#[macro_use]
pub mod lv;
mod sample;

use std::error::Error;
use crate::lv::server::Route;
use crate::sample::{SampleApp, SampleAppParams};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let route = Route::new("/hello");
	lv::server::start::<SampleApp>(SampleAppParams, route).await
}
