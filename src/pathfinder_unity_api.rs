// This file has been auto-generated, please do not edit it.

// pathfinder/c/src/lib.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! C bindings to Pathfinder.

use font_kit::handle::Handle;

use gl;
use pathfinder_canvas::{CanvasFontContext, CanvasRenderingContext2D, FillStyle, LineJoin};
use pathfinder_canvas::{Path2D, TextMetrics};
use pathfinder_content::color::{ColorF, ColorU};
use pathfinder_content::outline::ArcDirection;
use pathfinder_content::stroke::LineCap;
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_gpu::resources::{FilesystemResourceLoader, ResourceLoader};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_renderer::scene::Scene;
use pathfinder_simd::default::F32x4;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::slice;
use std::str;

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use metal::{CAMetalLayer, CoreAnimationLayerRef};
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
use pathfinder_metal::MetalDevice;

// Constants

// `canvas`

pub const PF_LINE_CAP_BUTT:   u8 = 0;
pub const PF_LINE_CAP_SQUARE: u8 = 1;
pub const PF_LINE_CAP_ROUND:  u8 = 2;

pub const PF_LINE_JOIN_MITER: u8 = 0;
pub const PF_LINE_JOIN_BEVEL: u8 = 1;
pub const PF_LINE_JOIN_ROUND: u8 = 2;

// `content`

pub const PF_ARC_DIRECTION_CW:  u8 = 0;
pub const PF_ARC_DIRECTION_CCW: u8 = 1;

// `renderer`

pub const PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR: u8 = 0x1;

// Types

// External: `font-kit`
pub type FKHandleRef = *mut Handle;

// `canvas`
pub type PFCanvasRef = *mut CanvasRenderingContext2D;
pub type PFPathRef = *mut Path2D;
pub type PFCanvasFontContextRef = *mut CanvasFontContext;
pub type PFFillStyleRef = *mut FillStyle;
pub type PFLineCap = u8;
pub type PFLineJoin = u8;
pub type PFArcDirection = u8;
#[repr(C)]
pub struct PFTextMetrics {
    pub width: f32,
}

// `content`
#[repr(C)]
pub struct PFColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
pub struct PFColorU {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// `geometry`
#[repr(C)]
pub struct PFVector2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
pub struct PFVector2I {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
pub struct PFRectF {
    pub origin: PFVector2F,
    pub lower_right: PFVector2F,
}
#[repr(C)]
pub struct PFRectI {
    pub origin: PFVector2I,
    pub lower_right: PFVector2I,
}

// `gl`
pub type PFGLDeviceRef = *mut GLDevice;
pub type PFGLVersion = GLVersion;
pub type PFGLFunctionLoader = extern "stdcall" fn(name: *const c_char, userdata: *mut c_void)
                                            -> *const c_void;
// `gpu`
pub type PFGLDestFramebufferRef = *mut DestFramebuffer<GLDevice>;
pub type PFGLRendererRef = *mut Renderer<GLDevice>;
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalDestFramebufferRef = *mut DestFramebuffer<MetalDevice>;
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalRendererRef = *mut Renderer<MetalDevice>;
// FIXME(pcwalton): Double-boxing is unfortunate. Remove this when `std::raw::TraitObject` is
// stable?
pub type PFResourceLoaderRef = *mut Box<dyn ResourceLoader>;

// `metal`
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
pub type PFMetalDeviceRef = *mut MetalDevice;

// `renderer`
pub type PFSceneRef = *mut Scene;
pub type PFSceneProxyRef = *mut SceneProxy;
#[repr(C)]
pub struct PFRendererOptions {
    pub background_color: PFColorF,
    pub flags: PFRendererOptionsFlags,
}
pub type PFRendererOptionsFlags = u8;
// TODO(pcwalton)
#[repr(C)]
pub struct PFBuildOptions {
    pub placeholder: u32,
}

// `canvas`

/// Consumes the font context.
#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasCreate(font_context: PFCanvasFontContextRef,
                                        size: *const PFVector2F)
                                        -> PFCanvasRef {
    Box::into_raw(Box::new(CanvasRenderingContext2D::new(*Box::from_raw(font_context),
                                                         (*size).to_rust())))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasDestroy(canvas: PFCanvasRef) {
    drop(Box::from_raw(canvas))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFontContextCreateWithSystemSource() -> PFCanvasFontContextRef {
    Box::into_raw(Box::new(CanvasFontContext::from_system_source()))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFontContextCreateWithFonts(fonts: *const FKHandleRef,
                                                            font_count: usize)
                                                            -> PFCanvasFontContextRef {
    let fonts = slice::from_raw_parts(fonts, font_count);
    Box::into_raw(Box::new(CanvasFontContext::from_fonts(fonts.into_iter().map(|font| {
        (**font).clone()
    }))))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFontContextDestroy(font_context: PFCanvasFontContextRef) {
    drop(Box::from_raw(font_context))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFontContextClone(font_context: PFCanvasFontContextRef)
                                                  -> PFCanvasFontContextRef {
    Box::into_raw(Box::new((*font_context).clone()))
}

/// Consumes the canvas.
#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasCreateScene(canvas: PFCanvasRef) -> PFSceneRef {
    Box::into_raw(Box::new(Box::from_raw(canvas).into_scene()))
}

// Drawing rectangles

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFillRect(canvas: PFCanvasRef, rect: *const PFRectF) {
    (*canvas).fill_rect((*rect).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasStrokeRect(canvas: PFCanvasRef, rect: *const PFRectF) {
    (*canvas).stroke_rect((*rect).to_rust())
}

// Drawing text

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFillText(canvas: PFCanvasRef,
                                          string: *const c_char,
                                          string_len: usize,
                                          position: *const PFVector2F) {
    (*canvas).fill_text(to_rust_string(&string, string_len), (*position).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasStrokeText(canvas: PFCanvasRef,
                                            string: *const c_char,
                                            string_len: usize,
                                            position: *const PFVector2F) {
    (*canvas).stroke_text(to_rust_string(&string, string_len), (*position).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasMeasureText(canvas: PFCanvasRef,
                                             string: *const c_char,
                                             string_len: usize,
                                             out_text_metrics: *mut PFTextMetrics) {
    debug_assert!(!out_text_metrics.is_null());
    *out_text_metrics = (*canvas).measure_text(to_rust_string(&string, string_len)).to_c()
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetLineWidth(canvas: PFCanvasRef, new_line_width: f32) {
    (*canvas).set_line_width(new_line_width)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetLineCap(canvas: PFCanvasRef, new_line_cap: PFLineCap) {
    (*canvas).set_line_cap(match new_line_cap {
        PF_LINE_CAP_SQUARE => LineCap::Square,
        PF_LINE_CAP_ROUND  => LineCap::Round,
        _                  => LineCap::Butt,
    });
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetLineJoin(canvas: PFCanvasRef, new_line_join: PFLineJoin) {
    (*canvas).set_line_join(match new_line_join {
        PF_LINE_JOIN_BEVEL => LineJoin::Bevel,
        PF_LINE_JOIN_ROUND => LineJoin::Round,
        _                  => LineJoin::Miter,
    });
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetMiterLimit(canvas: PFCanvasRef, new_miter_limit: f32) {
    (*canvas).set_miter_limit(new_miter_limit);
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetLineDash(canvas: PFCanvasRef,
                                             new_line_dashes: *const f32,
                                             new_line_dash_count: usize) {
    (*canvas).set_line_dash(slice::from_raw_parts(new_line_dashes, new_line_dash_count).to_vec())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetLineDashOffset(canvas: PFCanvasRef, new_offset: f32) {
    (*canvas).set_line_dash_offset(new_offset)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetFontByPostScriptName(canvas: PFCanvasRef,
                                                         postscript_name: *const c_char,
                                                         postscript_name_len: usize) {
    (*canvas).set_font_by_postscript_name(to_rust_string(&postscript_name, postscript_name_len))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetFontSize(canvas: PFCanvasRef, new_font_size: f32) {
    (*canvas).set_font_size(new_font_size)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetFillStyle(canvas: PFCanvasRef, fill_style: PFFillStyleRef) {
    (*canvas).set_fill_style(*fill_style)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasSetStrokeStyle(canvas: PFCanvasRef,
                                                stroke_style: PFFillStyleRef) {
    (*canvas).set_stroke_style(*stroke_style)
}

/// Consumes the path.
#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasFillPath(canvas: PFCanvasRef, path: PFPathRef) {
    (*canvas).fill_path(*Box::from_raw(path))
}

/// Consumes the path.
#[no_mangle]
pub unsafe extern "stdcall" fn PFCanvasStrokePath(canvas: PFCanvasRef, path: PFPathRef) {
    (*canvas).stroke_path(*Box::from_raw(path))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathCreate() -> PFPathRef {
    Box::into_raw(Box::new(Path2D::new()))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathDestroy(path: PFPathRef) {
    drop(Box::from_raw(path))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathClone(path: PFPathRef) -> PFPathRef {
    Box::into_raw(Box::new((*path).clone()))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathMoveTo(path: PFPathRef, to: *const PFVector2F) {
    (*path).move_to((*to).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathLineTo(path: PFPathRef, to: *const PFVector2F) {
    (*path).line_to((*to).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathQuadraticCurveTo(path: PFPathRef,
                                                ctrl: *const PFVector2F,
                                                to: *const PFVector2F) {
    (*path).quadratic_curve_to((*ctrl).to_rust(), (*to).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathBezierCurveTo(path: PFPathRef,
                                             ctrl0: *const PFVector2F,
                                             ctrl1: *const PFVector2F,
                                             to: *const PFVector2F) {
    (*path).bezier_curve_to((*ctrl0).to_rust(), (*ctrl1).to_rust(), (*to).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathArc(path: PFPathRef,
                                   center: *const PFVector2F,
                                   radius: f32,
                                   start_angle: f32,
                                   end_angle: f32,
                                   direction: PFArcDirection) {
    let direction = if direction == 0 { ArcDirection::CW } else { ArcDirection::CCW };
    (*path).arc((*center).to_rust(), radius, start_angle, end_angle, direction)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathArcTo(path: PFPathRef,
                                     ctrl: *const PFVector2F,
                                     to: *const PFVector2F,
                                     radius: f32) {
    (*path).arc_to((*ctrl).to_rust(), (*to).to_rust(), radius)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathRect(path: PFPathRef, rect: *const PFRectF) {
    (*path).rect((*rect).to_rust())
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathEllipse(path: PFPathRef,
                                       center: *const PFVector2F,
                                       axes: *const PFVector2F,
                                       rotation: f32,
                                       start_angle: f32,
                                       end_angle: f32) {
    (*path).ellipse((*center).to_rust(), (*axes).to_rust(), rotation, start_angle, end_angle)
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFPathClosePath(path: PFPathRef) {
    (*path).close_path()
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFFillStyleCreateColor(color: *const PFColorU) -> PFFillStyleRef {
    Box::into_raw(Box::new(FillStyle::Color((*color).to_rust())))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFFillStyleDestroy(fill_style: PFFillStyleRef) {
    drop(Box::from_raw(fill_style))
}

// `gl`

#[no_mangle]
pub unsafe extern "stdcall" fn PFFilesystemResourceLoaderLocate() -> PFResourceLoaderRef {
    let loader = Box::new(FilesystemResourceLoader::locate());
    Box::into_raw(Box::new(loader as Box<dyn ResourceLoader>))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLLoadWith(loader: PFGLFunctionLoader, userdata: *mut c_void) {
    gl::load_with(|name| {
        let name = CString::new(name).unwrap();
        loader(name.as_ptr(), userdata)
    });
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLDeviceCreate(version: PFGLVersion, default_framebuffer: u32)
                                          -> PFGLDeviceRef {
    Box::into_raw(Box::new(GLDevice::new(version, default_framebuffer)))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLDeviceDestroy(device: PFGLDeviceRef) {
    drop(Box::from_raw(device))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFResourceLoaderDestroy(loader: PFResourceLoaderRef) {
    drop(Box::from_raw(loader))
}

// `gpu`

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLDestFramebufferCreateFullWindow(window_size: *const PFVector2I)
                                                             -> PFGLDestFramebufferRef {
    Box::into_raw(Box::new(DestFramebuffer::full_window((*window_size).to_rust())))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLDestFramebufferDestroy(dest_framebuffer: PFGLDestFramebufferRef) {
    drop(Box::from_raw(dest_framebuffer))
}

/// Takes ownership of `device` and `dest_framebuffer`, but not `resources`.
#[no_mangle]
pub unsafe extern "stdcall" fn PFGLRendererCreate(device: PFGLDeviceRef,
                                            resources: PFResourceLoaderRef,
                                            dest_framebuffer: PFGLDestFramebufferRef,
                                            options: *const PFRendererOptions)
                                            -> PFGLRendererRef {
    Box::into_raw(Box::new(Renderer::new(*Box::from_raw(device),
                                         &**resources,
                                         *Box::from_raw(dest_framebuffer),
                                         (*options).to_rust())))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLRendererDestroy(renderer: PFGLRendererRef) {
    drop(Box::from_raw(renderer))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFGLRendererGetDevice(renderer: PFGLRendererRef) -> PFGLDeviceRef {
    &mut (*renderer).device
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalDestFramebufferCreateFullWindow(window_size: *const PFVector2I)
                                                                -> PFMetalDestFramebufferRef {
    Box::into_raw(Box::new(DestFramebuffer::full_window((*window_size).to_rust())))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalDestFramebufferDestroy(dest_framebuffer:
                                                       PFMetalDestFramebufferRef) {
    drop(Box::from_raw(dest_framebuffer))
}

/// Takes ownership of `device` and `dest_framebuffer`, but not `resources`.
#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalRendererCreate(device: PFMetalDeviceRef,
                                               resources: PFResourceLoaderRef,
                                               dest_framebuffer: PFMetalDestFramebufferRef,
                                               options: *const PFRendererOptions)
                                               -> PFMetalRendererRef {
    Box::into_raw(Box::new(Renderer::new(*Box::from_raw(device),
                                         &**resources,
                                         *Box::from_raw(dest_framebuffer),
                                         (*options).to_rust())))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalRendererDestroy(renderer: PFMetalRendererRef) {
    drop(Box::from_raw(renderer))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalRendererGetDevice(renderer: PFMetalRendererRef) -> PFMetalDeviceRef {
    &mut (*renderer).device
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFSceneProxyBuildAndRenderGL(scene_proxy: PFSceneProxyRef,
                                                      renderer: PFGLRendererRef,
                                                      build_options: *const PFBuildOptions) {
    (*scene_proxy).build_and_render(&mut *renderer, (*build_options).to_rust())
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFSceneProxyBuildAndRenderMetal(scene_proxy: PFSceneProxyRef,
                                                         renderer: PFMetalRendererRef,
                                                         build_options: *const PFBuildOptions) {
    (*scene_proxy).build_and_render(&mut *renderer, (*build_options).to_rust())
}

// `metal`

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalDeviceCreate(layer: *mut CAMetalLayer)
                                             -> PFMetalDeviceRef {
    Box::into_raw(Box::new(MetalDevice::new(CoreAnimationLayerRef::from_ptr(layer))))
}

#[cfg(all(target_os = "macos", not(feature = "pf-gl")))]
#[no_mangle]
pub unsafe extern "stdcall" fn PFMetalDeviceDestroy(device: PFMetalDeviceRef) {
    drop(Box::from_raw(device))
}

// `renderer`

#[no_mangle]
pub unsafe extern "stdcall" fn PFSceneDestroy(scene: PFSceneRef) {
    drop(Box::from_raw(scene))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFSceneProxyCreateFromSceneAndRayonExecutor(scene: PFSceneRef)
                                                                     -> PFSceneProxyRef {
    Box::into_raw(Box::new(SceneProxy::from_scene(*Box::from_raw(scene), RayonExecutor)))
}

#[no_mangle]
pub unsafe extern "stdcall" fn PFSceneProxyDestroy(scene_proxy: PFSceneProxyRef) {
    drop(Box::from_raw(scene_proxy))
}

// Helpers for `canvas`

unsafe fn to_rust_string(ptr: &*const c_char, mut len: usize) -> &str {
    if len == 0 {
        len = libc::strlen(*ptr);
    }
    str::from_utf8(slice::from_raw_parts(*ptr as *const u8, len)).unwrap()
}

trait TextMetricsExt {
    fn to_c(&self) -> PFTextMetrics;
}

impl TextMetricsExt for TextMetrics {
    fn to_c(&self) -> PFTextMetrics {
        PFTextMetrics { width: self.width }
    }
}

// Helpers for `content`

impl PFColorF {
    #[inline]
    pub fn to_rust(&self) -> ColorF {
        ColorF(F32x4::new(self.r, self.g, self.b, self.a))
    }
}

impl PFColorU {
    #[inline]
    pub fn to_rust(&self) -> ColorU {
        ColorU { r: self.r, g: self.g, b: self.b, a: self.a }
    }
}

// Helpers for `geometry`

impl PFRectF {
    #[inline]
    pub fn to_rust(&self) -> RectF {
        RectF::from_points(self.origin.to_rust(), self.lower_right.to_rust())
    }
}

impl PFRectI {
    #[inline]
    pub fn to_rust(&self) -> RectI {
        RectI::from_points(self.origin.to_rust(), self.lower_right.to_rust())
    }
}

impl PFVector2F {
    #[inline]
    pub fn to_rust(&self) -> Vector2F {
        Vector2F::new(self.x, self.y)
    }
}

impl PFVector2I {
    #[inline]
    pub fn to_rust(&self) -> Vector2I {
        Vector2I::new(self.x, self.y)
    }
}

// Helpers for `renderer`

impl PFRendererOptions {
    pub fn to_rust(&self) -> RendererOptions {
        let has_background_color = self.flags & PF_RENDERER_OPTIONS_FLAGS_HAS_BACKGROUND_COLOR;
        RendererOptions {
            background_color: if has_background_color != 0 {
                Some(self.background_color.to_rust())
            } else {
                None
            },
        }
    }
}

impl PFBuildOptions {
    pub fn to_rust(&self) -> BuildOptions {
        BuildOptions::default()
    }
}
