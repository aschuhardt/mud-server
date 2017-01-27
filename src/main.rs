//main.rs

// REMOVE ME BEFORE TESTING
#![allow(dead_code)]
#![allow(unused_imports)]
// REMOVE ME BEFORE TESTING

extern crate rustc_serialize;
extern crate uuid;

mod session;
mod configuration;
mod file_io;

use configuration::Configuration;
use session::Session;

fn main() {
	let conf = Configuration::load();
	println!("{}", conf.data_location);
}
