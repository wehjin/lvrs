#[macro_use]
extern crate serde_json;

use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
	lv::app::start().await
}

pub mod lv;
mod app;