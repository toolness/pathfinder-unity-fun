use std::env;
use std::path::{PathBuf};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

struct PluginState {
    logfile: PathBuf
}

impl PluginState {
    pub fn new() -> Self {
        let mut logfile = env::current_dir().unwrap();
        logfile.push("pathfinder-plugin.log");
        let mut plugin = PluginState {
            logfile,
        };
        plugin.log("Pathfinder plugin initialized.");
        plugin
    }

    pub fn log(&mut self, msg: &str) {
        if !self.logfile.exists() {
            File::create(&self.logfile).unwrap();
        }
        let mut file = OpenOptions::new().append(true).open(&self.logfile).unwrap();
        file.write(msg.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
        file.flush().unwrap();
    }
}

impl Drop for PluginState {
    fn drop(&mut self) {
        self.log("Shutting down Pathfinder plugin.");
    }
}

static mut PLUGIN_STATE: Option<PluginState> = None;

// This is called by Unity when the plugin is loaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginLoad(_ptr: *mut ::libc::c_void) {
    unsafe {
        assert!(PLUGIN_STATE.is_none());
        PLUGIN_STATE = Some(PluginState::new());
    }
}

// This is called by Unity when the plugin is about to be unloaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginUnload() {
    unsafe {
        assert!(PLUGIN_STATE.is_some());
        PLUGIN_STATE = None;
    }
}

fn get_plugin_state_mut() -> &'static mut PluginState {
    unsafe {
        match PLUGIN_STATE {
            None => panic!("Expected plugin to be initialized!"),
            Some(ref mut state) => state
        }
    }
}

#[no_mangle]
pub extern "stdcall" fn boop_stdcall(x: i32) -> i32 {
    get_plugin_state_mut();
    51 + x
}
