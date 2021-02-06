//use libc;
use sha1::{Sha1, Digest};

extern crate base64;

pub const READ_BUF_SIZE: usize = 128;
pub const WRITE_BUF_SIZE: usize = 128;

pub fn gen_key(websocket_key:Option<&String>) -> String {
    let mut sign = match websocket_key {
        Some (key) => { key.clone() }
        None => { String::from("") }
    };

    sign.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    let mut sha = Sha1::default();
    sha.input(sign.as_ref());

    let res = sha.result();
    
    base64::encode(res.as_slice())
}

/*pub fn is_reuseaddr(socket:i32) -> Result<bool, i32> {
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
}*/

/*pub fn set_reusaddr(socket:i32, yes:bool) -> Result<(), i32> {
    unsafe {
        let mut val:u32 = if yes { 1 } else { 0 };
        let optval = (&mut val) as *mut u32 as *mut libc::c_void;
        let optlen:u32 = 4u32;

        if libc::setsockopt(socket, libc::SOL_SOCKET, libc::SO_REUSEADDR, optval, optlen) == -1 {
            return Err(*libc::__errno_location());
        } 
        
        Ok(())
    }        
}*/

