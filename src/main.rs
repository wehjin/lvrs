#[macro_use]
extern crate serde_json;

pub mod lv;
mod sample;

use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
	lv::server::start().await
}
