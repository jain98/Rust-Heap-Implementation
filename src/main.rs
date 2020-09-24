mod heap;

fn main() {
    let mut input:[i32; 8] = [1,2,3,4,5,6,7,8];

    let heap_impl = heap::Heap::new(&mut input);

    heap_impl.build_heap()
}
