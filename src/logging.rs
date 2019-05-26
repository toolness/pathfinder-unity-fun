use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub struct Logger {
    logfile: PathBuf,
}

impl Logger {
    pub fn new(logfile: PathBuf) -> Self {
        Logger { logfile }
    }

    // TODO: Consider adding a locking mechanism around this to ensure that
    // it's thread-safe. Since Unity's renderer may be multi-threaded, who
    // knows where we might be called from.
    //
    // This should probably be &mut self, but that would probably introduce
    // massive borrowing headaches, so nah.
    pub fn log<T: AsRef<str>>(&self, msg: T) {
        if !self.logfile.exists() {
            File::create(&self.logfile).unwrap();
        }
        let mut file = OpenOptions::new().append(true).open(&self.logfile).unwrap();
        file.write(msg.as_ref().as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        file.flush().unwrap();
    }
}
