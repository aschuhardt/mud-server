//main.rs

// REMOVE ME BEFORE TESTING
#![allow(dead_code)]
#![allow(unused_imports)]
// REMOVE ME BEFORE TESTING

extern crate rustc_serialize;
extern crate uuid;

mod configuration;
mod file_io;
mod session;
mod message;
mod request;
mod remote_client;

use configuration::Configuration;

fn main() {
	let conf = Configuration::load();
	println!("{}", conf.data_location);
}
