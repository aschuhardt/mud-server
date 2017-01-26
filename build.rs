use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	create_config();
}

fn create_config() {
	let config_dir: &'static str = "config/";
	let config_filename: &'static str = "config.json";

	let config_full_path = format!("{}{}", config_dir, config_filename);
	let config_str = &config_full_path.as_str();
	let config_path = Path::new(config_str);

	//create config directory
	let config_path_display = config_path.display();
	if let Err(why) = fs::create_dir_all(&config_dir) {
		panic!("couldn't create directories for {}: {}", config_path_display,
													     why.description());
	}

	//create config file
	let mut file = match File::create(&config_path) {
    	Err(why) => panic!("couldn't create {}: {}", config_path_display,
        	                                         why.description()),
    	Ok(file) => file,
	};

	//write default config contents to file
	let config_contents = "{\"data_location\":\"data/data.json\",\"network_port\":10722}";
	if let Err(why) = file.write_all(config_contents.as_bytes()) {
            panic!("couldn't write to {}: {}", config_path_display,
                                               why.description());
	}
}
