pub trait Log: /*Sync + Send*/ {
    fn log(&self, s:&str) -> ();

    fn disable(&self) -> ();
    fn is_enable(&self) -> bool;
}

/*
pub struct SimpleLogger {
    pub enabled: bool
}

impl super::Log for StdoutLogger {
    fn log(&self, s:&str) {
        println!{"{}", s}
    }

    fn disable(&self) {
        //self.enabled = false
    }

    fn is_enable(&self) -> bool {
        self.enabled
    }
}

pub static mut LOGGER:&'static dyn super::Log = &StdoutLogger { 
    enabled: true 
};
*/

/*
//plug another 
static mut S:&'static dyn Log = &SimpleLogger::StdoutLogger {     
    enabled: false
};

unsafe { SimpleLogger::LOGGER = S; } 
*/

use std::{sync::Mutex, sync::Arc};

struct StdoutLogger {
    enabled: Arc<Mutex<bool>>
}

impl Log for StdoutLogger {
    fn log(&self, s:&str) {
        println!{"{}", s}
    }

    fn disable(&self) {
        let m = Arc::clone(&self.enabled);
        let mut enabled = m.lock().unwrap();
        *enabled = false;
    }

    fn is_enable(&self) -> bool {
        *self.enabled.lock().unwrap()
    }
}

use std::fs::File;
use std::io::Write;

pub struct FileLogger {
    enabled: Arc<Mutex<bool>>,
    file: Arc<Mutex<File>>
}

impl FileLogger {
    pub fn new(s:&str) -> FileLogger {
        FileLogger {
            enabled: Arc::new(Mutex::new(true)),
            file: Arc::new(Mutex::new(File::create(s).unwrap()))
        }
    }
}

impl Log for FileLogger {
    fn log(&self, s:&str) {
        match self.file.lock() {
            Ok(mut file) => {
                let _ = file.write_all(s.as_bytes());
                ()
            },
            _ => {}
        }
    }

    fn disable(&self) {
        let m = Arc::clone(&self.enabled);
        let mut enabled = m.lock().unwrap();
        *enabled = false;
    }

    fn is_enable(&self) -> bool {
        *self.enabled.lock().unwrap()
    }
}

lazy_static! {
    pub static ref LOGGER:Box<(dyn Log + Sync)> = Box::new(StdoutLogger { 
        enabled: Arc::new(Mutex::new(true))
    });
}