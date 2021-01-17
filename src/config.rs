use std::collections::HashMap;
use std::fs::File;
use std::io::{self, ErrorKind};
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
    path_aliases:HashMap<String, String>
}

impl Config{

    pub fn load(config_path: &String) -> Config{
        
        let contents = Config::read_config_from_file(&config_path).unwrap_or_else(|error| {
            panic!("Problem reading the file contents: {:?}", error)
        });

        return serde_json::from_str(&contents).expect("Config file is bad formatted.");
    }

    pub fn new() -> Config{

        let mut path_aliases = HashMap::new();
        path_aliases.insert(String::from("/downloads"), String::from("d"));
        path_aliases.insert(String::from("/documents"), String::from("doc"));
        return Config{
            path_aliases
        }
    }

    fn read_config_from_file(config_path: &String) -> Result<String, io::Error>{

        let mut f = File::open(config_path).unwrap_or_else(|error| {
            if error.kind() ==  ErrorKind::NotFound{
                File::create(config_path).unwrap_or_else(|error| {
                    panic!("Problem creating config file at {}:{:?}", config_path, error);
                })
            }else{
                panic!("Problem opening the file: {:?}", error);
            }
        });

        let mut contents = String::new();
        
        match f.read_to_string(&mut contents){
            Ok(_) => Ok(contents),
            Err(e) => Err(e),
        }

    }

    pub fn write_config_to_file(&self){
        serde_json::to_writer(File::create("config.json").unwrap(), &self);
    }
}