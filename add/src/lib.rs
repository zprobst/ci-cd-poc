pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

mod test {
    use super::add;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2))
    }
}
