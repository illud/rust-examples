pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);
}
