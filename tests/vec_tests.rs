use std::alloc;

#[test]
fn it_allocates() {
    let vec = fallible_alloc::vec::alloc_with_size::<f64, alloc::System>(alloc::System, 10).unwrap();
    assert_eq!(vec.len(), 10);
}
