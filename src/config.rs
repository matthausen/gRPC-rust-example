extern crate serde_derive;
extern crate serde_yaml;
use serde::{Serialize, Deserialize};

use std::{env};
use std::error::Error;

use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    something: bool,
    dynamodb: DynamoDBData
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamoDBData {
    table: String,
    endpoint: String,
}

pub fn load_env(key: &str) -> Result<String, Box<dyn Error>> {
    let environment: String;

    match env::var(key) {
        Ok(val) => environment = val,
        Err(e) => panic!("${} is not set ({})", key, e)
    }

    Ok(environment)
}


pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let mut content = String::new();
    let config: Config;

    match load_env("ENV") {
        Ok(env) => {
            let path: String = format!("{}/config/environment/{}.yml", std::env::current_dir().unwrap().display(), env);
                match File::open(path) {
                    Ok(mut file) => {
                        file.read_to_string(&mut content).unwrap();
                        config = serde_yaml::from_str(&content).unwrap();
                        println!("{:?}", config);
                    }
                    Err(error) => {
                        panic!("Could not load config file. Error {}", error)
                    }
                }
        }
        Err(error) => {
            panic!("Could not load environment variable. Error {}", error)
        }
    }

    Ok(config)
}