//configuration.rs

use rustc_serialize::json;
use file_io::FileIO;

static DEFAULT_CONFIG_PATH: &'static str = "config/config.json";

#[derive(RustcDecodable, RustcEncodable)]
pub struct Configuration {
    //add more fields to this as needed
    pub data_location: String,
    pub network_port: u16,
    pub debug_mode: bool,
    pub max_request_cache_count: usize,
    pub request_validation_token: String,
}

impl Configuration {
    pub fn load() -> Configuration {
        Configuration::load_path(DEFAULT_CONFIG_PATH)
    }

    pub fn load_path(path: &'static str) -> Configuration {
        let conf_json = FileIO::read_string(path);
        match json::decode(&conf_json) {
            Ok(c) => c,
            Err(why) => panic!("Unable to read supplied config file at {}: {}", path, why),
        }
    }

    #[cfg(test)]
    pub fn save_config(conf: &Configuration) {
        Configuration::save_config_path(conf, DEFAULT_CONFIG_PATH);
    }

    #[cfg(test)]
    pub fn save_config_path(conf: &Configuration, path: &str) {
        let conf_json = json::encode(conf).unwrap();
        FileIO::write_string(path, conf_json.as_str());
    }
}

#[cfg(test)]
mod tests {
    use configuration::Configuration;
    use file_io::FileIO;

    #[test]
    fn test_config_io() {
        let test_config_path = "testconfig.json";
        let test_conf = Configuration {
            data_location: "data/data.json".to_string(),
            network_port: 10722,
            debug_mode: true,
            max_request_cache_count: 64,
            request_validation_token: "d5695226-2508-4187-b1eb-bed9665fbf26".to_string(),
        };
        //serialize + save configuration object
        Configuration::save_config_path(&test_conf, test_config_path);
        //deserialize + read configuration object
        let loaded_conf = Configuration::load_path(test_config_path);
        //test whether the loaded file has the test data in it
        assert_eq!(test_conf.data_location, loaded_conf.data_location);
        assert_eq!(test_conf.network_port, loaded_conf.network_port);
        //clean up test file
        FileIO::delete_file(test_config_path);
    }
}
