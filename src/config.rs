extern crate yaml_rust;
use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Color {
    Black,
    White,
}

pub fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);

    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

pub const CONFIG_PATH: &str = "./src/config/config.yaml";

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &str = "./src/config/test.yaml";

    #[test]
    fn test_config() {
        let docs = load_yaml(TEST);
        let doc = &docs[0];
        // Test
        assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
        assert_eq!(doc["bar"][0].as_i64().unwrap(), 1);
        assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);
        assert!(doc["INVALID_KEY"][100].is_badvalue());
    }
}
