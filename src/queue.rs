pub struct Queue<T> {
    queue: Vec<T>,
    maxSize: u32,
}

impl<T> Queue<T> {
    fn new(maxSize: u32) -> Self {
        Queue {
            queue: Vec::new(),
            maxSize,
        }
    }

    fn add(&mut self, item: T) {
        self.queue.push(item);
        if self.queue.len() > self.maxSize {
            self.queue.remove(0)
        };
    }
}
