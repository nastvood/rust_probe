//#![feature(async_await)]
//#![feature(async_closure)]

use libc;
use std::error::Error;
use std::net::{ TcpListener, TcpStream };
use std::io;
use std::os::unix::io::AsRawFd;

use futures::{ 
    prelude::*,
    stream,
};

enum Event {
    Connection(io::Result<TcpStream>),
}

use crate::config::Config;

#[derive(Debug)]
pub struct Server {
    config: Config
}

fn is_reuseaddr(socket:i32) -> Result<bool, i32> {
    unsafe {
        let mut val:u32 = 0;
        let mut len:u32 = 4;

        let optval = (&mut val) as *mut u32 as *mut libc::c_void;
        let optlen = (&mut len) as *mut libc::socklen_t;
        if libc::getsockopt(socket, libc::SOL_SOCKET, libc::SO_REUSEADDR, optval, optlen) == -1 {
            return Err(*libc::__errno_location());
        } 

        if val == 1 { Ok(true) } else { Ok(false) }
    }
}

fn set_reusaddr(socket:i32, yes:bool) -> Result<(), i32> {
    unsafe {
        let mut val:u32 = if yes { 1 } else { 0 };
        let optval = (&mut val) as *mut u32 as *mut libc::c_void;
        let optlen:u32 = 4u32;

        if libc::setsockopt(socket, libc::SOL_SOCKET, libc::SO_REUSEADDR, optval, optlen) == -1 {
            return Err(*libc::__errno_location());
        } 
        
        Ok(())
    }        
}

impl Server {
    pub fn new(config:Config) -> Server {
        Server {
            config
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {        
        let addr = self.config.host.to_owned() + ":" + &self.config.port.to_string();

        let listener = TcpListener::bind(addr)?;
        eprintln!("Listening on {}", listener.local_addr()?);

        if !(is_reuseaddr(listener.as_raw_fd()).unwrap()) {
            set_reusaddr(listener.as_raw_fd(), true).unwrap();
        }

        let connections = listener.incoming().map(|stream| { Event::Connection(stream)});        

        let mut events = stream::iter(connections);

        loop {
            match events.next().await {
                Some (Event::Connection(Ok(stream))) => {
                    stream.set_nonblocking(true)?;

                    println!("{:?}", &stream);
                }
                _ => panic!("Event error"),
            }
        }

        //Ok(())
    }
}
