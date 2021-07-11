#[test]
fn it_allocates() {
    let vec: Vec<f64> = fallible_alloc::vec::alloc_with_size(10).unwrap();
    assert_eq!(vec.len(), 10);
}
