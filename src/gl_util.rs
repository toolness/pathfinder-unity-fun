use libc::c_void;
use std::ffi::{CString, CStr};
use std::collections::HashMap;
use winapi::um::wingdi::{wglGetProcAddress, wglGetCurrentContext, wglGetCurrentDC, wglMakeCurrent};
use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress};
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::{HGLRC, HDC};
use winapi::shared::ntdef::NULL;
use gl::types::*;

const OPENGL32_DLL: &'static [u8] = b"opengl32.dll\0";

pub type Context = HGLRC;

pub struct ContextSwitcher {
    orig_ctx: HGLRC,
    orig_dc: HDC,
}

impl ContextSwitcher {
    pub fn try_new(ctx: HGLRC, dc: HDC) -> Option<Self> {
        let orig_ctx = unsafe { wglGetCurrentContext() };
        let orig_dc = unsafe { wglGetCurrentDC() };
        info!("Switching to GL context {:?} using HDC {:?}.", ctx, dc);
        let success = unsafe { wglMakeCurrent(dc, ctx) };
        if success == TRUE {
            init();
            Some(ContextSwitcher { orig_ctx, orig_dc })
        } else {
            info!("Switch failed!");
            None
        }
    }
}

impl Drop for ContextSwitcher {
    fn drop(&mut self) {
        unsafe {
            let dc = self.orig_dc;
            let success = wglMakeCurrent(dc, self.orig_ctx);
            if success == TRUE {
                info!("Switched back to original GL context {:?}.", dc);
                init();
            } else {
                info!("Could not switch back to original GL context {:?}!", dc);
            }
        }
    }
}

pub struct ContextWatcher {
    current_context: HGLRC,
    context_dcs: HashMap<HGLRC, HDC>
}

impl ContextWatcher {
    pub fn new() -> Self {
        init();
        ContextWatcher {
            current_context: get_current_context(),
            context_dcs: HashMap::new()
        }
    }

    pub fn switch_to(&mut self, ctx: Context) -> Option<ContextSwitcher> {
        if let Some(hdc) = self.context_dcs.get(&ctx) {
            ContextSwitcher::try_new(ctx, *hdc)
        } else {
            info!("Unable to switch to GL context {:?} because no HDC for it exists.", ctx);
            None
        }
    }

    pub fn check(&mut self) -> Context {
        let ctx = get_current_context();
        if ctx != self.current_context {
            self.current_context = ctx;
            let hdc = unsafe { wglGetCurrentDC() };
            if hdc as *const c_void != NULL {
                self.context_dcs.insert(ctx, hdc);
            }
            init();
        }
        self.current_context
    }
}

pub fn get_current_context() -> HGLRC {
    unsafe {
        wglGetCurrentContext()
    }
}

fn get_proc_address(name: &str) -> *const c_void {
    let mut ptr;
    let cstring = CString::new(name).unwrap();
    unsafe {
        // https://www.khronos.org/opengl/wiki/Load_OpenGL_Functions
        ptr = wglGetProcAddress(cstring.as_ptr()) as *mut _;
        if ptr == NULL {
            let hmodule = LoadLibraryA(OPENGL32_DLL.as_ptr() as *const i8);
            assert_ne!(hmodule, NULL as *mut _);
            ptr = GetProcAddress(hmodule, cstring.as_ptr()) as *mut _;
        }
    }
    return ptr;
}

fn init() {
    gl::load_with(get_proc_address);
}

pub fn get_version() -> (GLint, GLint) {
    let mut major: GLint = 0;
    let mut minor: GLint = 0;

    unsafe {
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
    }

    (major, minor)
}

pub fn get_version_string() -> String {
    unsafe {
        let ptr = gl::GetString(gl::VERSION);
        let cstr = CStr::from_ptr(ptr as *const i8);
        cstr.to_string_lossy().to_string()
    }
}

pub fn get_viewport_size() -> (GLint, GLint, GLint, GLint) {
    let mut ints: [GLint; 4] = [0; 4];
    unsafe {
        gl::GetIntegerv(gl::VIEWPORT, ints.as_mut_ptr());
    }
    (ints[0], ints[1], ints[2], ints[3])
}

pub fn get_draw_framebuffer_binding() -> GLuint {
    let mut fbo_id: GLint = 0;

    unsafe {
        gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut fbo_id);
    }

    return fbo_id as GLuint;
}

pub fn set_draw_framebuffer_binding(fbo_id: GLuint) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
        assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
}
