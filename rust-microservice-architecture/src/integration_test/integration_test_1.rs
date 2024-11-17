#[test]
fn test_case_1() {
    use microservice_1::module_1::add;
    assert_eq!(add(1, 2), 3);
}
