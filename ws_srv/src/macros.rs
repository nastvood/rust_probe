#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { 
        unsafe {
            let s = format!{ "{}:{}: {}", file!{}, line!{}, format!{ $($arg)* } };
            super::LOGGER.log(&s[..]);
        }
    };
}
