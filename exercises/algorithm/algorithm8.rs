#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { elements: Vec::new() }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &'static str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0)) // Remove from the front
        } else {
            Err("Queue is empty")
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack { q1: Queue::new(), q2: Queue::new() }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Transfer all elements from q1 to q2 except the last one
        while self.q1.elements.len() > 1 {
            let value = self.q1.dequeue()?;
            self.q2.enqueue(value);
        }

        // Pop the last element from q1
        let top = self.q1.dequeue()?;

        // Move elements back to q1 from q2
        while let Ok(value) = self.q2.dequeue() {
            self.q1.enqueue(value);
        }

        Ok(top)
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
