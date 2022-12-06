use std::collections::HashSet;

struct SignalBuffer {
    buffer: Vec<char>,
    size: usize,
    head: usize,
    is_full: bool,
}

impl SignalBuffer {
    fn new(size: usize) -> SignalBuffer {
        SignalBuffer {
            buffer: Vec::with_capacity(size),
            size,
            head: 0,
            is_full: false,
        }
    }

    fn push(&mut self, c: char) {
        if self.buffer.len() == self.size {
            self.buffer[self.head] = c;
            self.is_full = true;
        } else {
            self.buffer.push(c);
        }
        self.head = (self.head + 1) % self.size;
    }
}

fn check_buffer_elements_are_distinct(buffer: &Vec<char>) -> bool {
    let mut s: HashSet<char> = HashSet::with_capacity(buffer.capacity());
    for c in buffer {
        if s.contains(c) {
            return false;
        }
        s.insert(*c);
    }
    true
}

fn find_distinct_buffered_sequence(input: &String, mut buffer: SignalBuffer) -> usize {
    for (i, c) in input.chars().enumerate() {
        buffer.push(c);
        if buffer.is_full && check_buffer_elements_are_distinct(&buffer.buffer) {
            return i + 1;
        }
    }
    0
}

pub fn solve(input: String) -> (String, String) {
    let result1 = find_distinct_buffered_sequence(&input, SignalBuffer::new(4));
    let result2 = find_distinct_buffered_sequence(&input, SignalBuffer::new(14));

    (
        format!("Number of characters processed for packet: {result1}"),
        format!("Number of characters processed for message: {result2}"),
    )
}
