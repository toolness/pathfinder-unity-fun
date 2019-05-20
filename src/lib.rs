use std::env;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

mod unity_interfaces;

use unity_interfaces::IUnityInterfaces;

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

    pub fn log<T: AsRef<str>>(&mut self, msg: T) {
        if !self.logfile.exists() {
            File::create(&self.logfile).unwrap();
        }
        let mut file = OpenOptions::new().append(true).open(&self.logfile).unwrap();
        file.write(msg.as_ref().as_bytes()).unwrap();
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
pub extern "stdcall" fn UnityPluginLoad(ptr: *mut IUnityInterfaces) {
    unsafe {
        assert!(PLUGIN_STATE.is_none());
        let mut plugin = PluginState::new();
        let gfx = (*ptr).get_unity_graphics();
        plugin.log(format!("Unity graphics is {:?}.", gfx));
        PLUGIN_STATE = Some(plugin);
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
    let plugin = get_plugin_state_mut();
    plugin.log(format!("boop_stdcall({}) called.", x));
    51 + x
}
