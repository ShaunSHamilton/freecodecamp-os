use std::fs::read_to_string;

use config::FreeCodeCampConf;

#[test]
fn parse() {
    let config_str = read_file("../example/freecodecamp.conf.json");
    let config: FreeCodeCampConf = serde_json::from_str(&config_str).unwrap();

    assert_eq!(config.addr.to_string(), "0.0.0.0:8080");
}

fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}
