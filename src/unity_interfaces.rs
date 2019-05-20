// This is essentially a Rust version of the following header files:
//
// https://bitbucket.org/Unity-Technologies/graphicsdemos/src/default/NativeRenderingPlugin/PluginSource/source/Unity/IUnityInterface.h
// https://bitbucket.org/Unity-Technologies/graphicsdemos/src/default/NativeRenderingPlugin/PluginSource/source/Unity/IUnityGraphics.h

use libc::{
    c_ulonglong,
    c_void
};

pub const I_UNITY_GRAPHICS_GUID: UnityInterfaceGUID = UnityInterfaceGUID {
    high: 0x7CBA0A9CA4DDB544,
    low: 0x8C5AD4926EB17B11
};

#[repr(C)]
pub struct UnityInterfaceGUID {
    pub high: c_ulonglong,
    pub low: c_ulonglong,
}

#[repr(C)]
pub struct IUnityInterfaces {
    // I have no idea how to put a struct on the stack in an ABI, so I'm not going to use
    // the non-split versions of the API.
    get_interface: *const c_void,
    register_interface: *const c_void,

    get_interface_split: extern "stdcall" fn(guid_high: c_ulonglong, guid_low: c_ulonglong) -> *const c_void,
    // TODO: Fill in the call signature for this function.
    register_interface_split: *const c_void,
}

impl IUnityInterfaces {
    pub fn get_unity_graphics(&self) -> *const c_void {
        (self.get_interface_split)(I_UNITY_GRAPHICS_GUID.high, I_UNITY_GRAPHICS_GUID.low)
    }
}
