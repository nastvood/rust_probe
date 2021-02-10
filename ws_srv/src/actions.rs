use serde::{ Deserialize, Serialize };

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

#[derive(Debug, Serialize)]
#[serde(rename = "respMessage")]
#[serde(tag = "type", rename_all(serialize = "camelCase"))]
pub struct RespMessage {
    pub from: String,
    pub message: String
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    #[serde(rename(deserialize = "login"))]
    Login(Login),
    #[serde(rename(deserialize = "message"))]
    Message(Message),
    Error
}

impl From<&str> for Action {
    fn from(data: &str) -> Self {
        log!("{:?}", data);

        match serde_json::from_str(data) {
            Ok(action) => action,
            _ => Action::Error
        }
    }
}
