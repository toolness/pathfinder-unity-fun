struct PluginState;

static mut PLUGIN_STATE: Option<PluginState> = None;

// This is called by Unity when the plugin is loaded.
#[no_mangle]
pub extern "stdcall" fn UnityPluginLoad(_ptr: *mut ::libc::c_void) {
    unsafe {
        assert!(PLUGIN_STATE.is_none());
        PLUGIN_STATE = Some(PluginState {});
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

fn get_plugin_state_mut() -> &'static mut Option<PluginState> {
    unsafe {
        return &mut PLUGIN_STATE;
    }
}

#[no_mangle]
pub extern "stdcall" fn boop_stdcall(x: i32) -> i32 {
    match get_plugin_state_mut() {
        None => -1,
        Some(_) => 50 + x
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::boop_stdcall;
        assert_eq!(boop_stdcall(2), -1);
    }
}
