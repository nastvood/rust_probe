use serde::{ Deserialize };
use serde_json:: { Result, Value };

#[derive(Debug, Deserialize)]
pub struct Login {
    login: String,
    pass: String
}

#[derive(Debug, Deserialize)]
pub struct Message {
    //to: String, //common chat
    message: String
}

#[derive(Debug)]
pub enum Action {
    Login(Login),
    Message(Message),
    Error
}

impl Action {
    pub fn from_str(data:&str) -> Result<Action> {
        let v:Value = serde_json::from_str(data)?;

        match (&v["action"], &v["data"]) {
            (Value::String(action), obj) => {
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
