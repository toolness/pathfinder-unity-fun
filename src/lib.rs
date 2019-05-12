use pathfinder_geometry::basic::point::{Point2DI32, Point2DF32};
use pathfinder_geometry::basic::rect::RectF32;
use pathfinder_canvas::{CanvasRenderingContext2D, Path2D};

#[no_mangle]
pub extern fn boop(x: i32) -> i32 {
    // https://github.com/pcwalton/pathfinder/blob/master/examples/canvas_minimal/src/main.rs
    let window_size = Point2DI32::new(640, 480);
    let mut canvas = CanvasRenderingContext2D::new(window_size.to_f32());
    canvas.set_line_width(10.0);

    // Draw walls.
    canvas.stroke_rect(RectF32::new(
        Point2DF32::new(75.0, 140.0),
        Point2DF32::new(150.0, 110.0)
    ));

    // Draw door.
    canvas.fill_rect(RectF32::new(
        Point2DF32::new(130.0, 190.0),
        Point2DF32::new(40.0, 60.0)
    ));

    // Draw roof.
    let mut path = Path2D::new();
    path.move_to(Point2DF32::new(50.0, 140.0));
    path.line_to(Point2DF32::new(150.0, 60.0));
    path.line_to(Point2DF32::new(250.0, 140.0));
    path.close_path();
    canvas.stroke_path(path);

    window_size.x() + x
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::boop;
        assert_eq!(boop(2), 642);
    }
}
