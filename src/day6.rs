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

/// This is a solution I saw on the Kotlin AoC livestream, which solves the problem in O(n) time,
/// as oppose to O(n*m) time (for n = input length, m = window size). I didn't want to use it as
/// my final solution, since I didn't come up with it myself, but I did want to implement it :)
fn _solve_optimal(input: &String, window_size: usize) -> usize {
    let mut last_seen_index = [0; 26];
    let mut most_recent_duplicate_index = 0;
    for (i, c) in input.chars().enumerate() {
        let ci = c as usize - 'a' as usize;
        let last_seen_c_index = last_seen_index[ci];
        last_seen_index[ci] = i;
        if last_seen_c_index > most_recent_duplicate_index {
            most_recent_duplicate_index = last_seen_c_index
        }
        if i - most_recent_duplicate_index >= window_size {
            return i + 1;
        }
    }
    0
}
