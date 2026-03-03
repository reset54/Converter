use std::sync::{Arc, Mutex};
pub type LogHandle = Arc<Mutex<Vec<String>>>;
pub struct Logger;


impl Logger {
    pub fn new_handle() -> LogHandle {
        Arc::new(Mutex::new(Vec::new()))
    }


    pub fn log(handle: &LogHandle, message: impl Into<String>) {
        if let Ok(mut lg) = handle.lock() {
            let msg = message.into();

            // Console output for debug
            println!("[LOG]: {}", msg);

            lg.push(msg);

            if lg.len() > 1000 {
                lg.remove(0);
            }
        }
    }
}
