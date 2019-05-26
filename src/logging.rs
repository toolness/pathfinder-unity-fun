use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::sync::{Mutex, Once};
use std::env;

static INIT: Once = Once::new();

static mut GLOBAL_LOGGER: Option<Logger> = None;

struct Logger {
    logfile: PathBuf,
    mutex: Mutex<()>
}

impl Logger {
    pub fn new(logfile: PathBuf) -> Self {
        Logger { logfile, mutex: Mutex::new(()) }
    }

    pub fn log<T: AsRef<str>>(&mut self, msg: T) {
        // Since Unity's renderer may be multi-threaded, who
        // knows where we might be called from, so we'll use a mutex just in case.
        let _ = self.mutex.lock().unwrap();

        if !self.logfile.exists() {
            File::create(&self.logfile).unwrap();
        }
        let mut file = OpenOptions::new().append(true).open(&self.logfile).unwrap();
        file.write(msg.as_ref().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        file.flush().unwrap();
    }
}

pub fn log<T: AsRef<str>>(msg: T) {
    INIT.call_once(|| {
        let mut logfile = env::current_dir().unwrap();
        logfile.push("pathfinder-plugin.log");
        let logger = Logger::new(logfile);
        unsafe {
            GLOBAL_LOGGER = Some(logger);
        }
    });
    if let Some(logger) = unsafe { &mut GLOBAL_LOGGER } {
        logger.log(msg);
    } else {
        panic!("Expected logger to exist!");
    }
}
