#[no_mangle]
pub extern fn boop(x: i32) -> i32 {
    x + 5
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::boop;
        assert_eq!(boop(2), 7);
    }
}
