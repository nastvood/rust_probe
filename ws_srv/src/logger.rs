
pub trait Log: Sync + Send {
    fn log(&self, s:&str) -> ();
}

pub struct StdoutLogger;

impl Log for StdoutLogger {
    fn log(&self, s:&str) {
        println!{"{}", s}
    }
}

pub static mut LOGGER:&dyn Log = &StdoutLogger;
