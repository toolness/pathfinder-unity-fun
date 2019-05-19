static mut LOADS: i32 = 0;

// This is called by Unity when the plugin is loaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginLoad(_ptr: *mut ::libc::c_void) {
    unsafe { LOADS += 1; }
}

// This is called by Unity when the plugin is about to be unloaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginUnload() {
}

#[no_mangle]
pub extern "stdcall" fn boop_stdcall(x: i32) -> i32 {
    let loads = unsafe { LOADS };
    loads + x
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::boop_stdcall;
        assert_eq!(boop_stdcall(2), 2);
    }
}
