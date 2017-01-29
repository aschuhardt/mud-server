//main.rs

// REMOVE ME BEFORE TESTING
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
// REMOVE ME BEFORE TESTING

extern crate rustc_serialize;
extern crate uuid;
extern crate time;

mod session;
mod configuration;
mod file_io;

use configuration::Configuration;
use session::Session;

fn main() {
	let conf = Configuration::load();
	let s = Session::new(&conf);
    s.run();
}
