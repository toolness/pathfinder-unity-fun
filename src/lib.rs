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

enum PluginCommand {
    Shutdown,
    RenderCanvas(i32)
}

struct PluginState {
    // A pointer to Unity's plugin API that it gives us when our plugin
    // is first loaded.
    unity_interfaces: *const IUnityInterfaces,

    // The graphics backend that Unity is using.
    unity_renderer: Option<UnityGfxRenderer>,

    // This is where we put Canvases that are ready to be rendered.
    canvases: Mutex<HashMap<i32, Box<CanvasRenderingContext2D>>>,

    // Unity sometimes switches between different GL contexts, so we need to
    // check when the current context has changed and adapt accordingly.
    gl_context_watcher: Option<gl_util::ContextWatcher>,

    // We use a separate renderer for each GL context that Unity uses. Ideally
    // we'd have some separation between the kinds of resources that are shared
    // between contexts (programs, shaders, etc) and those that aren't
    // (framebuffers, vertex arrays, etc) so that we'd be able to manage resources
    // more efficiently, but Pathfinder doesn't support such granularity right now.
    renderers: HashMap<gl_util::Context, Renderer>,

    // The directory where Pathfinder's resources (shaders, textures, etc) are.
    resources_dir: PathBuf,

    // Whether our plugin has had any errors or not. If it has, most calls to
    // the plugin will be no-ops.
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
            renderers: HashMap::new(),
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
                if let Some(UnityGfxRenderer::OpenGLCore) = self.unity_renderer {
                    self.gl_context_watcher = Some(gl_util::ContextWatcher::new());
                    let (major, minor) = gl_util::get_version();
                    let version = gl_util::get_version_string();
                    info!("OpenGL version is {}.{} ({}).", major, minor, version);
                }
            },
            Some(UnityGfxDeviceEventType::Shutdown) => {
                self.execute_command(PluginCommand::Shutdown);
            },
            _ => {}
        }
    }

    pub fn set_canvas(&mut self, id: i32, canvas: Box<CanvasRenderingContext2D>) {
        self.canvases.lock().unwrap().insert(id, canvas);
    }

    fn execute_opengl_command(&mut self, cmd: PluginCommand) {
        let context_watcher = self.gl_context_watcher.as_mut()
            .expect("GL context watcher should exist!");
        let ctx = context_watcher.check();
        match cmd {
            PluginCommand::Shutdown => {
                // We'll first try shutting down the renderer for the current GL
                // context, if any, which should be straightforward.
                let renderer = self.renderers.remove(&ctx);
                if renderer.is_some() {
                    info!("Shutting down renderer for current GL context {:?}.", ctx);
                    drop(renderer);
                }
                // Shutting down the other renderers is trickier, because they're
                // bound to other GL contexts: we need to switch over to each
                // before shutting down it's associated renderer, and then switch
                // back to our original GL context (if any) once we're done.
                let keys: Vec<gl_util::Context> = self.renderers.keys().map(|k| *k).collect();
                for ctx in keys.iter() {
                    if let Some(ctx_switcher) = context_watcher.switch_to(*ctx) {
                        if let Some(renderer) = self.renderers.remove(&ctx) {
                            info!("Shutting down renderer for GL context {:?}.", ctx);
                            drop(renderer);
                        }
                        drop(ctx_switcher);
                    }
                }
            },
            PluginCommand::RenderCanvas(canvas_id) => {
                let resources_dir = &self.resources_dir;
                let renderer = self.renderers.entry(ctx)
                    .or_insert_with(|| {
                        info!("Creating a renderer for GL context {:?}.", ctx);
                        Renderer::new(resources_dir)
                    });
                if let Some(canvas) = self.canvases.lock().unwrap().remove(&canvas_id) {
                    renderer.render(canvas);
                } else {
                    info!("RenderCanvas called with nonexistent canvas id {}.", canvas_id);
                    self.errored = true;
                }
            }
        }
    }

    pub fn execute_command(&mut self, cmd: PluginCommand) {
        if self.errored {
            return;
        }
        match self.unity_renderer {
            Some(UnityGfxRenderer::OpenGLCore) => {
                self.execute_opengl_command(cmd);
            },
            _ => {
                info!(
                    "execute_command() called, but rendering backend {:?} is unsupported.",
                    self.unity_renderer
                );
                self.errored = true;
            }
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

extern "stdcall" fn handle_shutdown(_event_id: c_int) {
    get_plugin_state_mut().execute_command(PluginCommand::Shutdown);
}

extern "stdcall" fn handle_render_canvas(canvas_id: c_int) {
    get_plugin_state_mut().execute_command(PluginCommand::RenderCanvas(canvas_id));
}

#[no_mangle]
pub extern "stdcall" fn get_render_canvas_func() -> UnityRenderingEvent {
    handle_render_canvas
}

#[no_mangle]
pub extern "stdcall" fn get_shutdown_func() -> UnityRenderingEvent {
    handle_shutdown
}

#[no_mangle]
pub extern "stdcall" fn queue_canvas_for_rendering(canvas: PFCanvasRef, id: c_int) {
    get_plugin_state_mut().set_canvas(id, unsafe { Box::from_raw(canvas) });
}
