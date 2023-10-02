pub struct PriorityQueue<T> {
    queue: Vec<T>,
    max_size: usize,
}

impl<T> PriorityQueue<T> {
    pub fn new(max_size: usize) -> Self {
        Queue {
            queue: Vec::new(),
            max_size,
        }
    }

    pub fn add(&mut self, item: T) {
        self.queue.push(item);
        if self.queue.len() > self.max_size {
            self.queue.remove(0);
        };
    }

    pub fn peek(&self) -> Option<&T> {
        return self.queue.first();
    }
}
