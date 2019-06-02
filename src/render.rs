use std::path::PathBuf;
use pathfinder_canvas::{CanvasRenderingContext2D, Path2D};
use pathfinder_geometry::basic::point::{Point2DF32, Point2DI32};
use pathfinder_geometry::basic::rect::RectF32;
use pathfinder_gpu::resources::FilesystemResourceLoader;
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::gpu::renderer::DestFramebuffer;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::options::RenderOptions;
use pathfinder_renderer::gpu::renderer::Renderer as PathfinderRenderer;
use gl::types::GLuint;

use crate::logging::log;
use crate::gl_util::{get_viewport_size, get_draw_framebuffer_binding};

fn get_current_window_size() -> Point2DI32 {
    let (_, _, width, height) = get_viewport_size();
    Point2DI32::new(width, height)
}

fn build_renderer(
    window_size: Point2DI32,
    framebuffer: GLuint,
    loader: &FilesystemResourceLoader
) -> PathfinderRenderer<GLDevice> {
    pathfinder_renderer::gpu::renderer::Renderer::new(
        GLDevice::new(GLVersion::GL3, framebuffer),
        loader,
        DestFramebuffer::full_window(window_size)
    )
}

pub struct Renderer {
    renderer: PathfinderRenderer<GLDevice>,
    window_size: Point2DI32,
    framebuffer: GLuint
}

impl Renderer {
    pub fn new(resources_dir: PathBuf) -> Self {
        let window_size = get_current_window_size();
        let framebuffer = get_draw_framebuffer_binding();
        let loader = FilesystemResourceLoader { directory: resources_dir };
        let renderer = build_renderer(window_size, framebuffer, &loader);

        Renderer { renderer, window_size, framebuffer }
    }

    // If Unity's window size/framebuffer changes, make sure our draw
    // calls adapt.
    fn sync_gfx_state(&mut self) {
        let framebuffer = get_draw_framebuffer_binding();
        let window_size = get_current_window_size();
        if window_size != self.window_size || framebuffer != self.framebuffer {
            log(format!(
                "Window size/framebuffer changed from {:?}/{} to {:?}/{}.",
                self.window_size,
                self.framebuffer,
                window_size,
                framebuffer
            ));
            self.window_size = window_size;
            self.framebuffer = framebuffer;
            self.renderer.device.set_default_framebuffer(framebuffer);
            self.renderer.replace_dest_framebuffer(DestFramebuffer::full_window(window_size));
            self.renderer.set_main_framebuffer_size(window_size);
        }
    }

    pub fn render(&mut self) {
        self.sync_gfx_state();
        let renderer = &mut self.renderer;

        // Make a canvas. We're going to draw a house.
        let mut canvas = CanvasRenderingContext2D::new(self.window_size.to_f32());

        // Set line width.
        canvas.set_line_width(10.0);

        // Draw walls.
        canvas.stroke_rect(RectF32::new(Point2DF32::new(75.0, 140.0), Point2DF32::new(150.0, 110.0)));

        // Draw door.
        canvas.fill_rect(RectF32::new(Point2DF32::new(130.0, 190.0), Point2DF32::new(40.0, 60.0)));

        // Draw roof.
        let mut path = Path2D::new();
        path.move_to(Point2DF32::new(50.0, 140.0));
        path.line_to(Point2DF32::new(150.0, 60.0));
        path.line_to(Point2DF32::new(250.0, 140.0));
        path.close_path();
        canvas.stroke_path(path);

        // Render the canvas to screen.
        let scene = SceneProxy::from_scene(canvas.into_scene(), RayonExecutor);
        scene.build_and_render(renderer, RenderOptions::default());
    }
}
