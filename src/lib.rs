#[macro_use]
extern crate enum_primitive_derive;

#[macro_use]
extern crate log;

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashMap;
use libc::c_int;

mod unity_interfaces;
mod gl_util;
mod render;
mod logging;
pub mod pathfinder_unity_api;

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

struct PluginState {
    unity_interfaces: *const IUnityInterfaces,
    unity_renderer: Option<UnityGfxRenderer>,
    canvases: Mutex<HashMap<i32, Box<CanvasRenderingContext2D>>>,
    renderer: Option<Renderer>,
    resources_dir: PathBuf,
    gl_context_watcher: Option<gl_util::ContextWatcher>,
    errored: bool
}

impl PluginState {
    pub fn new(unity_interfaces: *const IUnityInterfaces) -> Self {
        let (errored, resources_dir) = match Self::find_resources_dir() {
            Some(resources_dir) => (false, resources_dir),
            None => (true, PathBuf::new())
        };
        if errored {
            info!("Unable to find resources dir.");
        }
        let mut plugin = PluginState {
            unity_interfaces,
            unity_renderer: None,
            canvases: Mutex::new(HashMap::new()),
            gl_context_watcher: None,
            renderer: None,
            resources_dir,
            errored
        };
        plugin.initialize();
        plugin
    }

    fn find_resources_dir() -> Option<PathBuf> {
        for dir_name in ["unity-project_Data", "Assets"].iter() {
            let mut resources_dir = env::current_dir().unwrap();
            resources_dir.push(dir_name);
            resources_dir.push("StreamingAssets");
            resources_dir.push("pathfinder");
            info!("Searching for resources at {}.", resources_dir.to_string_lossy());
            if resources_dir.exists() {
                return Some(resources_dir);
            }
        }
        None
    }

    fn initialize(&mut self) {
        info!("Pathfinder plugin initialized.");
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
        info!("Unity renderer is {:?}.", self.get_unity_renderer());
    }

    pub fn handle_unity_device_event(&mut self, event_type: Option<UnityGfxDeviceEventType>) {
        info!("Unity graphics event occurred: {:?}", event_type);
        match event_type {
            Some(UnityGfxDeviceEventType::Initialize) => {
                self.log_unity_renderer_info();
                self.unity_renderer = self.get_unity_renderer();
                info!("Unity renderer is {:?}.", self.unity_renderer);
                if let Some(UnityGfxRenderer::OpenGLCore) = self.unity_renderer {
                    self.gl_context_watcher = Some(gl_util::ContextWatcher::new());
                    let (major, minor) = gl_util::get_version();
                    let version = gl_util::get_version_string();
                    info!("OpenGL version is {}.{} ({}).", major, minor, version);
                }
            },
            _ => {}
        }
    }

    pub fn set_canvas(&mut self, id: i32, canvas: Box<CanvasRenderingContext2D>) {
        self.canvases.lock().unwrap().insert(id, canvas);
    }

    pub fn render(&mut self, canvas_id: i32) {
        if self.errored {
            return;
        }
        if let Some(UnityGfxRenderer::OpenGLCore) = self.unity_renderer {
            let context_watcher = self.gl_context_watcher.as_mut()
              .expect("GL context watcher should exist!");
            if context_watcher.changed() {
                self.renderer = None;
            }
            let resources_dir = &self.resources_dir;
            let renderer = self.renderer.get_or_insert_with(|| Renderer::new(resources_dir));
            if let Some(canvas) = self.canvases.lock().unwrap().remove(&canvas_id) {
                renderer.render(canvas);
            } else {
                info!("render() called with nonexistent canvas id {}.", canvas_id);
                self.errored = true;
            }
        } else {
            info!(
                "render() called, but rendering backend {:?} is unsupported.",
                self.unity_renderer
            );
            self.errored = true;
        }
    }
}

impl Drop for PluginState {
    fn drop(&mut self) {
        info!("Shutting down Pathfinder plugin.");
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
        logging::init();
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

extern "stdcall" fn handle_render_canvas(canvas_id: c_int) {
    get_plugin_state_mut().render(canvas_id);
}

#[no_mangle]
pub extern "stdcall" fn get_render_canvas_func() -> UnityRenderingEvent {
    handle_render_canvas
}

#[no_mangle]
pub extern "stdcall" fn queue_canvas_for_rendering(canvas: PFCanvasRef, id: c_int) {
    get_plugin_state_mut().set_canvas(id, unsafe { Box::from_raw(canvas) });
}
