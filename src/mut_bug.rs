fn func<T>(option: Option<T>) -> Option<T> {
    option
}

#[test]
fn test() {
    assert_eq!(func::<String>(None), None);
}
