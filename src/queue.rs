pub struct Queue<T> {
    head: usize,
    tail: usize,
    data: Vec<Option<T>>,
}

// TODO: implement proper queue.
