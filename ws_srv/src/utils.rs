//use libc;

pub const READ_BUF_SIZE: usize = 128;

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

