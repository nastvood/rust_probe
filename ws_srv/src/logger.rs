#[allow(dead_code)]
#[derive(Debug)]
enum Level {
    Log,
    Warning,
    Error,
    Info
}

pub trait Log: {
    fn log(&self, s:&str);

    fn disable(&self);
    fn is_enable(&self) -> bool;

    fn disable_color(&self);
    fn is_color(&self) -> bool;
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
    enabled: Arc<Mutex<bool>>,
    colored: Arc<Mutex<bool>> 
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


    fn disable_color(&self) {
        let m = Arc::clone(&self.colored);
        let mut colored = m.lock().unwrap();
        *colored = false;
    }

    fn is_color(&self) -> bool {
        *self.colored.lock().unwrap()
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
        if let Ok(mut file) = self.file.lock() {
            let _ = file.write("\n".as_bytes());
            let _ = file.write_all(s.as_bytes());
        }
    }

    fn disable(&self) {
        let m = Arc::clone(&self.enabled);
        let mut enabled = m.lock().unwrap();
        *enabled = false;
    }

    fn disable_color(&self) {}

    fn is_enable(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    fn is_color(&self) -> bool {
        false    
    }
}

lazy_static! {
    pub static ref LOGGER:Mutex<Vec<Box<(dyn Log + Sync + Send)>>> = Mutex::new(vec![
        Box::new(StdoutLogger { 
            enabled: Arc::new(Mutex::new(true)),
            colored: Arc::new(Mutex::new(true))
        })
    ]);

    pub static ref LOCAL_TIME_OFFSET:i64 = {
        let dt = chrono::Local::now();
        dt.offset().local_minus_utc() as i64
    };
}

pub fn set(new_logger:Box<dyn Log + Sync + Send >) {
    LOGGER.lock().unwrap().clear();

    LOGGER.lock().unwrap().push(new_logger);        
}

pub fn is_enable() -> bool {
    LOGGER.lock().unwrap()[0].is_enable()
}

pub fn disable() {
    LOGGER.lock().unwrap()[0].disable();
}

pub fn is_color() -> bool {
    LOGGER.lock().unwrap()[0].is_color()
}

pub fn disable_color() {
    LOGGER.lock().unwrap()[0].disable_color()
}

pub fn log(s:&str) {
    LOGGER.lock().unwrap()[0].log(s);
}
