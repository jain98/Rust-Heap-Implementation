#[path = "../src/heap.rs"]
pub mod heap;

#[test]
fn heap_sift_up() {
    // ARRANGE
    let mut input = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut heap = heap::Heap::new(&mut input);

    // ACT
    heap.sift_up(7);

    // ASSERT
    assert_eq!([8,1,3,2,5,6,7,4], heap.get_backing_array());
}

#[test]
fn heap_sift_down_recursive() {
    // ARRANGE
    let mut input = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut heap = heap::Heap::new(&mut input);

    // ACT
    heap.sift_down_recursive(0, 8);

    // ASSERT
    assert_eq!([3,2,7,4,5,6,1,8], heap.get_backing_array());
}

#[test]
fn heap_sift_down_iterative() {
    // ARRANGE
    let mut input = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut heap = heap::Heap::new(&mut input);

    // ACT
    heap.sift_down_iterative(0, 8);

    // ASSERT
    assert_eq!([3,2,7,4,5,6,1,8], heap.get_backing_array());
}
