//main.rs

extern crate mud_server_session;

use mud_server_session::configuration::Configuration;
use mud_server_session::session::Session;

fn main() {
	let conf = Configuration::load();
	let s = Session::new(&conf);
    s.run();
}
