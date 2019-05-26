#[macro_use]
extern crate enum_primitive_derive;

use std::env;
use std::path::PathBuf;
use libc::c_int;

mod unity_interfaces;
mod gl_util;
mod render;
mod logging;

use unity_interfaces::{
    IUnityGraphics,
    IUnityInterfaces,
    UnityGfxRenderer,
    UnityRenderingEvent,
    UnityGfxDeviceEventType,
    UnityGfxDeviceEventTypeInt
};
use render::Renderer;
use logging::log;

struct PluginState {
    unity_interfaces: *const IUnityInterfaces,
    unity_renderer: Option<UnityGfxRenderer>,
    renderer: Option<Renderer>,
    errored: bool
}

impl PluginState {
    pub fn new(unity_interfaces: *const IUnityInterfaces) -> Self {
        let mut plugin = PluginState {
            unity_interfaces,
            unity_renderer: None,
            renderer: None,
            errored: false
        };
        plugin.initialize();
        plugin
    }

    fn find_resources_dir(&mut self) -> Option<PathBuf> {
        let mut resources_dir = env::current_dir().unwrap();
        resources_dir.push("unity-project_Data");
        resources_dir.push("StreamingAssets");
        resources_dir.push("pathfinder");
        log(format!("Searching for resources at {}.", resources_dir.to_string_lossy()));
        if resources_dir.exists() {
            Some(resources_dir)
        } else {
            // TODO: Also look in the "Assets" folder, if we're being run
            // inside the Unity editor?
            None
        }
    }

    fn initialize(&mut self) {
        log("Pathfinder plugin initialized.");
        unsafe {
            self.log_unity_renderer_info();
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

    pub fn get_unity_renderer(&self) -> Option<UnityGfxRenderer> {
        let gfx = self.get_unity_graphics();
        unsafe {
            ((*gfx).get_renderer)().convert()
        }
    }

    pub fn log_unity_renderer_info(&mut self) {
        log(format!("Unity renderer is {:?}.", self.get_unity_renderer()));
    }

    pub fn handle_unity_device_event(&mut self, event_type: Option<UnityGfxDeviceEventType>) {
        log(format!("Unity graphics event occurred: {:?}", event_type));
        match event_type {
            Some(UnityGfxDeviceEventType::Initialize) => {
                self.log_unity_renderer_info();
                self.unity_renderer = self.get_unity_renderer();
                if let Some(UnityGfxRenderer::OpenGLCore) = self.unity_renderer {
                    gl_util::init();
                    let (major, minor) = gl_util::get_version();
                    let version = gl_util::get_version_string();
                    log(format!("OpenGL version is {}.{} ({}).", major, minor, version));
                }
            },
            _ => {}
        }
    }

    fn try_to_init_renderer(&mut self) {
        match self.find_resources_dir() {
            None => {
                log("Unable to find resources dir.");
                self.errored = true;
            },
            Some(resources_dir) => {
                self.renderer = Some(Renderer::new(resources_dir));
            }
        }
    }

    pub fn render(&mut self) {
        if self.errored {
            return;
        }
        if let Some(UnityGfxRenderer::OpenGLCore) = self.unity_renderer {
            if self.renderer.is_none() {
                self.try_to_init_renderer();
            }
            if let Some(renderer) = &mut self.renderer {
                renderer.render();
            }
        } else {
            log(format!(
                "render() called, but rendering backend {:?} is unsupported.",
                self.unity_renderer
            ));
            self.errored = true;
        }
    }
}

impl Drop for PluginState {
    fn drop(&mut self) {
        log("Shutting down Pathfinder plugin.");
        // TODO: We should ideally unregister our device event callback here, but
        // we never seem to actually get called, so I guess it's not that urgent...
    }
}

static mut PLUGIN_STATE: Option<PluginState> = None;

extern "stdcall" fn handle_unity_device_event(event_type: UnityGfxDeviceEventTypeInt) {
    get_plugin_state_mut().handle_unity_device_event(event_type.convert());
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

// TODO: Remove this function, we don't need it. But also
// remove any C# code that calls into it.
#[no_mangle]
pub extern "stdcall" fn boop_stdcall(x: i32) -> i32 {
    log(format!("boop_stdcall({}) called.", x));
    51 + x
}

extern "stdcall" fn handle_render_event(_event_id: c_int) {
    get_plugin_state_mut().render();
}

#[no_mangle]
pub extern "stdcall" fn get_render_event_func() -> UnityRenderingEvent {
    handle_render_event
}
