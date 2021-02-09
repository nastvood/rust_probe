macro_rules! cur_hours_minutes_seconds  {
    () => {{
        let dt = chrono::Local::now();
        let mut secs = dt.timestamp();
        let offset = dt.offset().local_minus_utc();
        secs += offset as i64;

        secs -= secs / 86400 * 86400;
        let hours = secs / 3600;
        let secs = secs - hours * 3600;
        let minutes = secs / 60;

        (hours, minutes, secs - minutes * 60)
    }}
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { 
        if super::logger::is_enable() {
            let (hours, minutes, seconds) = cur_hours_minutes_seconds!{};
            let s = format!{ "\x1b[1;35m[{}:{}]\x1b[0;34m[{:02}:{:02}:{:02}]\x1b[0m {}",file!{}, line!{}, hours, minutes, seconds, format!{ $($arg)* } };
            super::logger::log(&s[..]);
        }
    };
}
