#[macro_use]
extern crate enum_primitive_derive;

use std::env;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use libc::c_int;

mod unity_interfaces;
mod gl_util;

use unity_interfaces::{
    IUnityGraphics,
    IUnityInterfaces,
    UnityGfxRenderer,
    UnityRenderingEvent,
    UnityGfxDeviceEventType,
    UnityGfxDeviceEventTypeInt
};

struct PluginState {
    logfile: PathBuf,
    unity_interfaces: *const IUnityInterfaces
}

impl PluginState {
    pub fn new(unity_interfaces: *const IUnityInterfaces) -> Self {
        let mut logfile = env::current_dir().unwrap();
        logfile.push("pathfinder-plugin.log");
        let mut plugin = PluginState {
            logfile,
            unity_interfaces
        };
        plugin.initialize();
        plugin
    }

    fn find_resources_dir(&mut self) -> PathBuf {
        let mut resources_dir = env::current_dir().unwrap();
        resources_dir.push("unity-project_Data");
        resources_dir.push("StreamingAssets");
        resources_dir.push("pathfinder");
        self.log(format!("Searching for resources at {}.", resources_dir.to_string_lossy()));
        if resources_dir.exists() {
            self.log("Found resources dir!");
        } else {
            // TODO: Also look in the "Assets" folder, if we're being run
            // inside the Unity editor?
            self.log("Unable to find resources dir!");
        }
        resources_dir
    }

    fn initialize(&mut self) {
        self.log("Pathfinder plugin initialized.");
        self.find_resources_dir();
        unsafe {
            self.log_renderer_info();
            let gfx = self.get_unity_graphics();
            ((*gfx).register_device_event_callback)(handle_unity_device_event);
        }
    }

    fn get_unity_graphics(&self) -> *const IUnityGraphics {
        unsafe {
            let gfx = (*self.unity_interfaces).get_unity_graphics();
            assert!(!gfx.is_null());
            gfx
        }
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

    pub fn get_renderer(&self) -> Option<UnityGfxRenderer> {
        let gfx = self.get_unity_graphics();
        unsafe {
            ((*gfx).get_renderer)().convert()
        }
    }

    pub fn log_renderer_info(&mut self) {
        self.log(format!("Renderer is {:?}.", self.get_renderer()));
    }
}

impl Drop for PluginState {
    fn drop(&mut self) {
        self.log("Shutting down Pathfinder plugin.");
        // TODO: We should ideally unregister our device event callback here, but
        // we never seem to actually get called, so I guess it's not that urgent...
    }
}

static mut PLUGIN_STATE: Option<PluginState> = None;

extern "stdcall" fn handle_unity_device_event(event_type_int: UnityGfxDeviceEventTypeInt) {
    let plugin = get_plugin_state_mut();
    let event_type = event_type_int.convert();
    plugin.log(format!("Unity graphics event occurred: {:?}", event_type));
    match event_type {
        Some(UnityGfxDeviceEventType::Initialize) => {
            plugin.log_renderer_info();
            if plugin.get_renderer() == Some(UnityGfxRenderer::OpenGLCore) {
                gl_util::init();
                let (major, minor) = gl_util::get_version();
                let version = gl_util::get_version_string();
                plugin.log(format!("OpenGL version is {}.{} ({}).", major, minor, version));
                plugin.log(format!("Viewport size is {:?}.", gl_util::get_viewport_size()));
            }
        },
        _ => {}
    }
}

// This is called by Unity when the plugin is loaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginLoad(unity_interfaces: *const IUnityInterfaces) {
    unsafe {
        assert!(PLUGIN_STATE.is_none());
        let plugin = PluginState::new(unity_interfaces);
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

extern "stdcall" fn handle_render_event(event_id: c_int) {
    let plugin = get_plugin_state_mut();
    plugin.log(format!("handle_render_event({}) called.", event_id));
    plugin.log(format!("Viewport size is {:?}.", gl_util::get_viewport_size()));
}

#[no_mangle]
pub extern "stdcall" fn get_render_event_func() -> UnityRenderingEvent {
    handle_render_event
}
