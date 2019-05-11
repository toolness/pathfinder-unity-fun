use pathfinder_geometry::basic::point::{Point2DI32};

#[no_mangle]
pub extern fn boop(x: i32) -> i32 {
    let window_size = Point2DI32::new(640, 480);
    window_size.x() + x
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::boop;
        assert_eq!(boop(2), 7);
    }
}
