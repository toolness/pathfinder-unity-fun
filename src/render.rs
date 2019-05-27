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

use crate::logging::log;
use crate::gl_util::{get_viewport_size, get_draw_framebuffer_binding};

fn get_current_window_size() -> Point2DI32 {
    let (_, _, width, height) = get_viewport_size();
    Point2DI32::new(width, height)
}

fn build_renderer(
    window_size: Point2DI32,
    loader: &FilesystemResourceLoader
) -> PathfinderRenderer<GLDevice> {
    let fbo_id = get_draw_framebuffer_binding();
    pathfinder_renderer::gpu::renderer::Renderer::new(
        GLDevice::new(GLVersion::GL3, fbo_id),
        loader,
        DestFramebuffer::full_window(window_size)
    )
}

pub struct Renderer {
    loader: FilesystemResourceLoader,
    renderer: PathfinderRenderer<GLDevice>,
    window_size: Point2DI32
}

impl Renderer {
    pub fn new(resources_dir: PathBuf) -> Self {
        let window_size = get_current_window_size();
        let loader = FilesystemResourceLoader { directory: resources_dir };
        let renderer = build_renderer(window_size, &loader);

        Renderer { loader, renderer, window_size }
    }

    fn check_window_size(&mut self) {
        let window_size = get_current_window_size();
        if window_size != self.window_size {
            let fb = get_draw_framebuffer_binding();
            log(format!(
                "Window size changed from {:?} to {:?} w/ fb {}.",
                self.window_size,
                window_size,
                fb
            ));
            self.window_size = window_size;

            // When Unity changes its window size, it often also changes the
            // current draw framebuffer, and since Pathfinder doesn't currently
            // seem to support retargeting that, we need to create a whole new
            // renderer from scratch, which is a bummer.
            self.renderer = build_renderer(self.window_size, &self.loader);
        }
    }

    pub fn render(&mut self) {
        self.check_window_size();
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
        let scene = SceneProxy::new(canvas.into_scene(), RayonExecutor);
        scene.build_and_render(renderer, RenderOptions::default());
    }
}
