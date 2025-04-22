

pub struct Stack<T> {
    items: [T],
    size: usize,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        Self {
            items: [T; size],
            size,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.size < self.items.len() {
            self.items[self.size] = item;
            self.size += 1;
        } else {
            panic!("Stack overflow");
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.items[self.size])
        } else {
            None
        }
    }
}
