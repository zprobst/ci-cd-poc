pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

mod test {
    use super::*;
    #[test]
    fn test_sub() {
        assert_eq!(3, sub(5, 2))
    }
}
