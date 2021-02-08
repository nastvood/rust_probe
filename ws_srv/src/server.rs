use std::error::Error;
//use std::os::unix::io::AsRawFd;
use std::collections::HashMap;
use std::io::{ self, Write };
//use std::net::SocketAddr;

use mio::{Events, Interest, Poll, Token};
use mio::net::{ TcpListener };

use crate::config::Config;
use crate::client::Client;
use crate::utils::{ READ_BUF_SIZE };

const LISTENER: Token = Token(0);

#[derive(Debug)]
pub struct Server {
    config: Config,
    client_addr: HashMap<Token, Client>,
    next_token_index: usize
}

impl Server {
    pub fn new(config:Config) -> Server {
        Server {
            config,
            client_addr: HashMap::new(),
            next_token_index: LISTENER.into()
        }
    }

    fn get_next_token(&mut self) -> Token {
        self.next_token_index += 1;
        Token(self.next_token_index)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {        
        let addr = (self.config.host.to_owned() + ":" + &self.config.port.to_string()).parse()?;

        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let mut listener = TcpListener::bind(addr)?;
        log!{"listening on {}", listener.local_addr()?}

        poll.registry().register(&mut listener, LISTENER, Interest::READABLE)?;

        loop {

            poll.poll(&mut events, None)?;

            for event in events.iter() {
                match event.token() {
                    LISTENER => {
                        loop {
                            match listener.accept() {
                                Ok((mut client, addr))=> {
                                    log!("accept {:?}", addr);

                                    let token = self.get_next_token();
                                    poll.registry().register(&mut client, token, Interest::READABLE | Interest::WRITABLE)?;

                                    let client = Client::new(client, addr);
                
                                    self.client_addr.insert(token, client);
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                   // Socket is not ready anymore, stop accepting
                                    break;
                                }
                                e => panic!("err={:?}", e),    
                            }
                        }
                    }

                    token => {
                        loop {
                            match self.client_addr.get_mut(&token).unwrap().read() {
                                Ok(0) => {
                                    log!("remove token {:?}", &token);
                                    self.client_addr.remove(&token);
                                    break;
                                }
    
                                Ok(n) => {
                                    if n < READ_BUF_SIZE {
                                        match self.client_addr.get_mut(&token).unwrap().process_packet() {
                                            Ok(Some(resp)) => {
                                                for (_token, client) in self.client_addr.iter_mut() {
                                                    client.stream.write(resp.as_ref())?;
                                                }
                                            },
                                            _ => {}
                                        }

                                        break;
                                    }
                                }

                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    // Socket is not ready anymore, stop reading
                                    break;
                                }

                                e => panic!("err={:?}", e)
                            }
                        }
                    }
                }
            }
        }
    }
}
