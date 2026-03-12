pub struct MyPriorityQueue<T: PartialOrd> {
    data: Vec<T>
}

impl<T: PartialOrd> Default for MyPriorityQueue<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T: PartialOrd> MyPriorityQueue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.bubble_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { return None; }
        
        let result = self.data.swap_remove(0); // Swaps 0 with last and removes
        if !self.data.is_empty() {
            self.bubble_down(0);
        }
        Some(result)
    }

    pub fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] <= self.data[parent] { break; }
            self.data.swap(idx, parent);
            idx = parent;
        }
    }

    fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut largest = index;

            if left < self.data.len() && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < self.data.len() && self.data[right] > self.data[largest] {
                largest = right;
            }
            if largest == index { break; }
            
            self.data.swap(index, largest);
            index = largest;
        }
    }

    pub fn find(&self, item: &T) -> Option<usize> {
        self.data.iter().position(|x| x == item)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}