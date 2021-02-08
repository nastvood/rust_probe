use std::error::Error;
use mio::net::{ TcpStream };
use std::net::{ SocketAddr };
use std::collections::HashMap;
use std::io::{ self, Read, Write };

extern crate base64;

use crate::utils::*;
use crate::ws::{ Opcode, Frame };
use crate::actions::{ Action, Login, RespMessage };

#[derive(Debug)]
enum Status {
    AwaitingHandshake,
    AwaitingLogin,
    Loggined,
}

#[derive(Debug)]
pub struct Client {
    status: Status,
    pub stream: TcpStream,
    addr: SocketAddr,
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,
    header: HashMap<String, String>,
    login: Option<String>
}

impl Client {
    pub fn new(stream:TcpStream, addr:SocketAddr) -> Self {
        Client {
            status : Status::AwaitingHandshake,
            stream,
            addr,
            read_buf:Vec::with_capacity(READ_BUF_SIZE),
            write_buf:Vec::with_capacity(WRITE_BUF_SIZE),
            header: HashMap::new(),
            login: None
        }
    }

    pub fn read(&mut self) -> io::Result<usize> {
        let cur_len = self.read_buf.len();

        if self.read_buf.capacity() < cur_len + READ_BUF_SIZE {
            self.read_buf.reserve(cur_len + READ_BUF_SIZE)
        }
        
        unsafe {
            self.read_buf.set_len(cur_len + READ_BUF_SIZE);
        }

        match self.stream.read(&mut self.read_buf.as_mut_slice()[cur_len..(cur_len + READ_BUF_SIZE)]) {
            Ok(n) if n > 0 => {
                unsafe {                
                    self.read_buf.set_len(cur_len + n);
                }

                Ok(n)
            }

            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                unsafe {                
                    self.read_buf.set_len(cur_len);
                }

                Err(e)
            }


            v => { v }
        }
    }

    fn read_packet(&mut self) -> Result<Option<Action>, Box<dyn Error>> {
        match self.status {
            Status::AwaitingHandshake => {
                let s:&str = std::str::from_utf8(&self.read_buf)?;

                for data in s.split("\r\n\r\n") {
                    for line in data.split("\r\n") {
                        let mut first = None;

                        for part in line.splitn(2, ':') {
                            match first {
                                None => { 
                                    first = Some (part)
                                }
                                Some(k) => {
                                    //log!("[{}]:[{}]", k, part);

                                    let v = match part.strip_prefix(" ") {
                                        Some(v) => v.to_owned(),
                                        None => part.to_owned()
                                    };
                                    self.header.insert(k.to_lowercase(), v);
                                    first = None
                                }
                            }
                        }
                    }
                }

                self.read_buf.clear();

                log!("{:?}", self.header);

                Ok(None)
            }

            Status::AwaitingLogin => {
                let frame = Frame::new(&self.read_buf);

                match frame.opcode {
                    Opcode::Text(data) => {
                        let action = Action::from(&data[..]);

                        Ok(Some(action))
                    }
                    _ => Ok(None)
                }
            }

            Status::Loggined => {

                let frame = Frame::new(&self.read_buf);


                match frame.opcode {
                    Opcode::Text(data) => {
                        let action = Action::from(&data[..]);

                        Ok(Some(action))
                    }
                    _ => Ok(None)
                }
            }
        }
    }

    /*fn fill_write_buf(&mut self, s:&str) {
        unsafe {
            self.write_buf.set_len(s.len());
            std::ptr::copy(s.as_ptr(), self.write_buf.as_mut_ptr(), s.len());
        }
    }*/

    pub fn check_login(&self, login:&Login) -> bool {
        return !login.login.is_empty()
    }

    pub fn process_packet(&mut self) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        self.header.clear();

        let res = match self.status {
            Status::AwaitingHandshake => {
                self.read_packet()?; 

                let key = gen_key(self.header.get("sec-websocket-key"));

                let resp = format!("HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\nUpgrade: websocket\r\n\r\n", key);

                self.stream.write(resp.as_bytes())?;

                self.status = Status::AwaitingLogin;

                Ok(None)
            }

            Status::AwaitingLogin => {
                let packet:Option<Action> = self.read_packet()?;

                match packet {
                    Some(Action::Login(login)) => {
                        log!("{:?}", login);

                        if self.check_login(&login) {
                            self.login = Some(login.login);
                            self.status = Status::Loggined
                        }

                        Ok(None)
                    }, 

                    _ => Ok(None)
                }
            }

            Status::Loggined => {
                let packet:Option<Action> = self.read_packet()?;

                match packet {
                    Some(Action::Message(message)) => {
                        log!("{:?}", message);

                        let resp_message = RespMessage {
                            from: self.login.as_ref().unwrap().to_string(),
                            message: message.message
                        };

                        let json_mess = serde_json::to_string(&resp_message)?;
                        let frame:Frame = json_mess.as_str().into();
                        let res:Vec<u8> = Vec::from(&frame);
                        
                        Ok(Some(res))
                    }, 

                    _ => Ok(None)
                }
            }
        };

        self.read_buf.clear();

        res
    }
}
