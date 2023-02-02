struct Queue<T> {
    input_queue: Vec<T>,
    output_queue: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            input_queue: vec![],
            output_queue: vec![]
        }
    }
    fn push(&mut self, value: T) {
        self.input_queue.push(value);
    }
    fn pop(&mut self) -> Option<T> {
        if self.output_queue.is_empty() {
            self.cycle();
        }
        self.output_queue.pop()
    }
    fn cycle(&mut self) {
        while let Some(item) = self.input_queue.pop() {
            self.output_queue.push(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_VALUES: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    #[test]
    fn it_works() {
        let mut queue = Queue::new();
        for value in TEST_VALUES {
            queue.push(value);
        }
        let mut input = 0;
        while let Some(value) = queue.pop() {
            if let Some(expected) = TEST_VALUES.get(input) {
                assert_eq!(*expected, value, "INCORRECT VALUE!");
            } else {
                panic!("SHOULD NOT BE HERE!");
            }
            input += 1;
        }

    }
}
