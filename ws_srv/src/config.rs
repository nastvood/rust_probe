use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use crate::logger::*;

#[derive(Debug)]
pub struct Config {
  pub port: u16,
  pub host: String
}

const DEFALT_PORT:u16 = 8080;
const DEFAULT_HOST:&str = "127.0.0.1";

impl Default for Config {
    fn default() -> Self {
        Config {
            port: DEFALT_PORT,
            host: String::from(DEFAULT_HOST)
        }
    }
}

fn parse_args(args: &[String]) -> HashMap<String, Option<String>> {
    let mut hargs = HashMap::new();

    let mut key = None; 
    for a in args.iter().skip(1) {
        if a.starts_with("-") || a.starts_with("--") {
            key = Some(a);
            hargs.insert((*a).clone(), None);
        } else {
            match key {
                Some (k) => { 
                    let _ = hargs.insert((*k).clone(), Some((*a).clone()));
                    key = None 
                }
                None => {
                 //free argument is not used
                }
          }
        }
    }

    return hargs;
}

impl Config {

    pub fn by_args(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 1 {
            panic!("not enough arguments, at least 1")
        }

        let mut conf = Config::default();

//            LOGGER = Box::new(super::FileLogger::new("ws.log")) as super::LOGGER;

        for (key, val) in parse_args(args) {
            match (&key[..], val) {
                ("-p", Some(v)) => conf.port = u16::from_str(&v)?,
                ("-h", Some(v)) => conf.host = v.clone(),
                ("--disable-log", None) => { 
                    LOGGER.disable();                 
                },
                _ => {}
            }
        }

        Ok(conf)  
    }
}
