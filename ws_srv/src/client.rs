use std::error::Error;
use mio::net::{ TcpStream };
use std::net::{ SocketAddr };
use std::collections::HashMap;
use std::io::{ self, Read };

use crate::utils::{ READ_BUF_SIZE };

#[derive(Debug)]
enum Status {
    AwaitingHandshake,
}

#[derive(Debug)]
pub struct Client {
    status: Status,
    pub stream: TcpStream,
    addr: SocketAddr,
    buf: Vec<u8>,
    header: HashMap<String, String>,
}

impl Client {
    pub fn new(stream:TcpStream, addr:SocketAddr) -> Self {
        Client {
            status : Status::AwaitingHandshake,
            stream,
            addr,
            buf:Vec::with_capacity(READ_BUF_SIZE),
            header: HashMap::new() 
        }
    }

    pub fn read(&mut self) -> io::Result<usize> {
        let cur_len = self.buf.len();

        if self.buf.capacity() < cur_len + READ_BUF_SIZE {
            self.buf.reserve(cur_len + READ_BUF_SIZE)
        }
        
        unsafe {
            self.buf.set_len(cur_len + READ_BUF_SIZE);
        }

        match self.stream.read(&mut self.buf.as_mut_slice()[cur_len..(cur_len + READ_BUF_SIZE)]) {
            Ok(n) if n > 0 => {
                //println!(" ---- read {}", n);
                unsafe {                
                    self.buf.set_len(cur_len + n);
                }

                Ok(n)
            }

            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                unsafe {                
                    self.buf.set_len(cur_len);
                }

                Err(e)
            }


            v => { v }
        }
    }

    fn read_header(&mut self) -> Result<(), Box<dyn Error>> {
        match self.status {
            Status::AwaitingHandshake => {
                let s:&str = std::str::from_utf8(&self.buf)?;

                for data in s.split("\r\n\r\n") {
                    for line in data.split("\r\n") {
                        let mut first = None;

                        for part in line.splitn(2, ' ') {
                            match first {
                                None => { 
                                    first = Some (part)
                                }
                                Some(k) => {
                                    println!("[{}]:[{}]", k, part);
                                    self.header.insert(k.to_lowercase(), part.to_owned());
                                    first = None
                                }
                            }
                        }
                    }
                }

                self.buf.clear();

                println!("{:?}", self.header);

                Ok(())
            }
        }
    }

    fn gen_key(&self) {
        let mut sign = match self.header.get("sec-websocket-key") {
            Some (key) => { key.clone() }
            None => { String::from("") }
        };

        sign.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    }

    pub fn process_packet(&mut self) -> Result<(), Box<dyn Error>> {
        self.header.clear();

        match self.status {
            Status::AwaitingHandshake => {
                self.read_header()?;

                //println!("{:?}", String::from_utf8(self.buf.clone()));

            }
        }

        self.buf.clear();

        Ok(())
    }
}
