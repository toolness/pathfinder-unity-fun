#[macro_use]
extern crate enum_primitive_derive;

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use libc::c_int;

mod unity_interfaces;
mod gl_util;
mod render;
mod logging;
mod pathfinder_unity_api;

use pathfinder_canvas::CanvasRenderingContext2D;
use pathfinder_unity_api::PFCanvasRef;
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
    canvas: Mutex<Option<Box<CanvasRenderingContext2D>>>,
    renderer: Option<Renderer>,
    errored: bool
}

impl PluginState {
    pub fn new(unity_interfaces: *const IUnityInterfaces) -> Self {
        let mut plugin = PluginState {
            unity_interfaces,
            unity_renderer: None,
            canvas: Mutex::new(None),
            renderer: None,
            errored: false
        };
        plugin.initialize();
        plugin
    }

    fn find_resources_dir(&mut self) -> Option<PathBuf> {
        for dir_name in ["unity-project_Data", "Assets"].iter() {
            let mut resources_dir = env::current_dir().unwrap();
            resources_dir.push(dir_name);
            resources_dir.push("StreamingAssets");
            resources_dir.push("pathfinder");
            log(format!("Searching for resources at {}.", resources_dir.to_string_lossy()));
            if resources_dir.exists() {
                return Some(resources_dir);
            }
        }
        None
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
                log(format!("Unity renderer is {:?}.", self.unity_renderer));
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
                log(format!(
                    "Found resources dir at {}, initializing renderer.",
                    resources_dir.to_string_lossy()
                ));
                self.renderer = Some(Renderer::new(resources_dir));
            }
        }
    }

    pub fn set_canvas(&mut self, canvas: Box<CanvasRenderingContext2D>) {
        let mut locked_canvas = self.canvas.lock().unwrap();
        *locked_canvas = Some(canvas);
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
                if let Some(canvas) = self.canvas.lock().unwrap().take() {
                    renderer.render(canvas);
                }
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

extern "stdcall" fn handle_render_event(_event_id: c_int) {
    get_plugin_state_mut().render();
}

#[no_mangle]
pub extern "stdcall" fn get_render_event_func() -> UnityRenderingEvent {
    handle_render_event
}

#[no_mangle]
pub extern "stdcall" fn queue_canvas_for_rendering(canvas: PFCanvasRef) {
    get_plugin_state_mut().set_canvas(unsafe { Box::from_raw(canvas) });
}
