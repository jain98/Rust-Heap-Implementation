pub struct Heap<'a> {
    heap: &'a mut [i32],
}

impl<'a> Heap<'a> {
    pub fn new(input: &mut [i32]) -> Heap {
        Heap {
            heap: input
        }
    }

    pub fn print_heap(&self) {
        for x in self.heap.iter() {
            print!("{}, ", x);
        }
        println!();
    }

    pub fn heap_sort_recursive(&mut self) {
        let n = self.heap.len();
        println!("Sorted input is: ");

        for i in (0..n).rev() {
            print!("{}, ", self.heap[0]);
            self.swap(0, i);
            //self.sift_down_iterative(0, i);
            self.sift_down_recursive(0, i);
        }

        println!();
    }

    pub fn heap_sort_iterative(&mut self) {
        let n = self.heap.len();
        println!("Sorted input is: ");

        for i in (0..n).rev() {
            print!("{}, ", self.heap[0]);
            self.swap(0, i);
            self.sift_down_iterative(0, i);
        }

        println!();
    }

    pub fn build_heap_recursive(&mut self) {
        let n = self.heap.len();

        for i in (0..n/2).rev() {
            //self.sift_down_iterative(i, n);
            self.sift_down_recursive(i, n);
        }
        println!("Done building heap!");
    }

    pub fn build_heap_iterative(&mut self) {
        let n = self.heap.len();

        for i in (0..n/2).rev() {
            self.sift_down_iterative(i, n);
        }
        println!("Done building heap!");
    }

    pub fn sift_up(&mut self, index_to_sift_up: usize) {
        let mut item_to_sift_up = self.heap[index_to_sift_up];
        let mut c = index_to_sift_up;
        let mut p = (c-1) / 2;

        while self.heap[p] < item_to_sift_up && c > 0 {
            self.swap(p, c);
            c = p;
            if c > 0 {
                p = (c - 1) / 2;
            }
        }

        self.heap[c] = item_to_sift_up;
    }

    pub fn sift_down_iterative(&mut self, root_index: usize, heap_size: usize) {
        let mut p = root_index;
        let mut c1;
        let mut c2;

        while p < heap_size/2 {
            c1 = p * 2 + 1;
            c2 = p * 2 + 2;

            let mut largest = self.heap[p];
            let mut largest_index = p;

            if c1 < heap_size &&  largest < self.heap[c1] {
                largest = self.heap[c1];
                largest_index = c1;
            }
            if c2 < heap_size && largest < self.heap[c2] {
                largest = self.heap[c2];
                largest_index = c2;
            }

            if largest_index == p {
                break;
            }

            self.swap(largest_index, p);
            p = largest_index;
        }
    }

    pub fn sift_down_recursive(&mut self, root_index: usize, heap_size: usize) {
        let mut p = root_index;
        let mut c1 = p * 2 + 1;
        let mut c2 = p * 2 + 2;

        let mut largest = self.heap[p];
        let mut largest_index = p;

        if c1 < heap_size &&  largest < self.heap[c1] {
            largest = self.heap[c1];
            largest_index = c1;
        }
        if c2 < heap_size && largest < self.heap[c2] {
            largest = self.heap[c2];
            largest_index = c2;
        }

        if largest_index != p {
            self.swap(largest_index, p);
            self.sift_down_recursive(largest_index, heap_size);
        }
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        unsafe {
            let temp = self.heap[i1];
            self.heap[i1] = self.heap[i2];
            self.heap[i2] = temp;
        }
    }
}
