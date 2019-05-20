// This is essentially a Rust version of the following header files:
//
// https://bitbucket.org/Unity-Technologies/graphicsdemos/src/default/NativeRenderingPlugin/PluginSource/source/Unity/IUnityInterface.h
// https://bitbucket.org/Unity-Technologies/graphicsdemos/src/default/NativeRenderingPlugin/PluginSource/source/Unity/IUnityGraphics.h

use libc::{
    c_ulonglong,
    c_int,
    c_void
};

#[repr(C)]
pub struct UnityInterfaceGUID {
    pub high: c_ulonglong,
    pub low: c_ulonglong,
}

// Type alias to indicate that we don't need a struct member, so we won't bother writing
// a (potentially incorrect) typing for it. However, it does occupy the size of a C void
// pointer in a struct, so we want to declare it for memory layout purposes.
type Unneeded = *const c_void;

#[repr(C)]
pub struct IUnityInterfaces {
    // I have no idea how passing a struct by value works in FFI, so I'm not going to use
    // the non-split versions of the API.
    get_interface: Unneeded,
    register_interface: Unneeded,

    get_interface_split: extern "stdcall" fn(guid_high: c_ulonglong, guid_low: c_ulonglong) -> *const c_void,
    register_interface_split: Unneeded,
}

impl IUnityInterfaces {
    pub fn get_unity_graphics(&self) -> *const IUnityGraphics {
        (self.get_interface_split)(I_UNITY_GRAPHICS_GUID.high, I_UNITY_GRAPHICS_GUID.low) as *const IUnityGraphics
    }
}

#[derive(Debug)]
pub enum UnityGfxRenderer {
    // OpenGL            =  0, // Legacy OpenGL, removed
    // D3D9              =  1, // Direct3D 9, removed
    D3D11             =  2, // Direct3D 11
    GCM               =  3, // PlayStation 3
    Null              =  4, // "null" device (used in batch mode)
    OpenGLES20        =  8, // OpenGL ES 2.0
    OpenGLES30        = 11, // OpenGL ES 3.0
    GXM               = 12, // PlayStation Vita
    PS4               = 13, // PlayStation 4
    XboxOne           = 14, // Xbox One
    Metal             = 16, // iOS Metal
    OpenGLCore        = 17, // OpenGL core
    D3D12             = 18, // Direct3D 12
    Vulkan            = 21, // Vulkan
    Nvn               = 22, // Nintendo Switch NVN API
    XboxOneD3D12      = 23  // MS XboxOne Direct3D 12
}

#[derive(Debug)]
pub enum UnityGfxDeviceEventType
{
    Initialize     = 0,
    Shutdown       = 1,
    BeforeReset    = 2,
    AfterReset     = 3,
}

type IUnityGraphicsDeviceEventCallback = extern "stdcall" fn(event_type: UnityGfxDeviceEventType);

pub struct IUnityGraphics {
    pub get_renderer: extern "stdcall" fn() -> UnityGfxRenderer,
    pub register_device_event_callback: extern "stdcall" fn(cb: IUnityGraphicsDeviceEventCallback),
    pub unregister_device_event_callback: extern "stdcall" fn(cb: IUnityGraphicsDeviceEventCallback),
    pub reserve_event_id_range: extern "stdcall" fn(count: c_int) -> c_int
}

pub const I_UNITY_GRAPHICS_GUID: UnityInterfaceGUID = UnityInterfaceGUID {
    high: 0x7CBA0A9CA4DDB544,
    low: 0x8C5AD4926EB17B11
};
