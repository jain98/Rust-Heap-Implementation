pub struct Heap<'a> {
    heap: &'a mut [i32]
}

impl Heap<'_> {
    pub fn new(input: & mut [i32]) -> Self {
        Heap {
            heap: input
        }
    }

    pub fn print_heap(&self) {
        for x in self.heap {
            print!("{}, ", x);
        }
        println!();
    }

    pub fn build_heap(&mut self) {
        let n = self.heap.len();

        for i in (0..n/2 - 1).rev() {
            self.sift_down_iterative(i, n);
        }
        println!("Done building heap!");
    }

    pub fn sift_up(&mut self) {}

    pub fn sift_down_iterative(&mut self, root_index: usize, heap_size: usize) {
        let mut parent = root_index;
        let mut c1;
        let mut c2;

        while parent < heap_size/2 {
            parent = root_index;
            c1 = parent*2 +1;
            c2 = parent*2 + 2;

            let mut largest = self.heap[parent];
            let mut largest_index = parent;

            if largest < self.heap[c1] {
                largest = self.heap[c1];
                largest_index = c1;
            } else if largest < self.heap[c2] {
                largest = self.heap[c2];
                largest_index = c2;
            } else {
                break;
            }

            self.swap(largest_index, parent);
            parent = largest_index;
        }
    }

    pub fn sift_down_recursive(&mut self, root_index: usize, heap_size: usize) {

    }

    fn swap(&mut self, i1: usize, i2: usize) {
        unsafe {
            let temp = self.heap[i1];
            self.heap[i1] = self.heap[i2];
            self.heap[i2] = temp;
        }
    }
}
