use libc::c_void;
use std::ffi::{CString, CStr};
use winapi::um::wingdi::wglGetProcAddress;
use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress};
use winapi::shared::ntdef::NULL;
use gl::types::*;

const OPENGL32_DLL: &'static [u8] = b"opengl32.dll\0";

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

pub fn init() {
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
