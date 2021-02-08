use serde::{ Deserialize, Serialize };
use serde_json:: { Result, Value };

use std::convert::From;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub login: String,
    pub pass: String
}

#[derive(Debug, Deserialize)]
pub struct Message {
    //to: String, //common chat
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespMessage {
    pub from: String,
    pub message: String
}

#[derive(Debug)]
pub enum Action {
    Login(Login),
    Message(Message),
    Error
}

impl Action {
    pub fn from_str(data:&str) -> Result<Action> {
        println!("{}:{}: {:?}", file!(), line!(), data);

        let v:Value = serde_json::from_str(data)?;

        match (&v["action"], &v["data"]) {
            (Value::String(action), obj) => {
                println!("{}:{}: {:?} {:?}", file!(), line!(), action, obj);
                match action {
                    action if action == "login" => {
                        match serde_json::from_value(obj.to_owned()) {
                            Ok(l) => Ok(Action::Login(l)),
                             _ => Ok(Action::Error)
                        }
                    },
                    action if action == "message" => {
                        match serde_json::from_value(obj.to_owned()) {
                            Ok(m) => Ok(Action::Message(m)),
                             _ => Ok(Action::Error)
                        }
                    },
                    _ => Ok(Action::Error)
                }                
            },

            _ => Ok(Action::Error)
        }
    }
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match Action::from_str(s) {
            Ok(v) => v,
            Err(_) => Action::Error
        }

        //Action::Error
    }
}
