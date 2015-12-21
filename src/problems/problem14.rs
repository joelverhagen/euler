struct CollatzSequence {
    next: u64,
    done: bool,
}

impl CollatzSequence {
    fn iter(start: u64) -> CollatzSequence {
        CollatzSequence { next: start, done: false }
    }
}

impl Iterator for CollatzSequence {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.done {
            None
        } else {
            let current = self.next;
            if current == 1 {
                self.done = true;
            }

            if current % 2 == 0 {
                self.next = self.next / 2;
            } else {
                self.next = (3 * self.next) + 1;
            }

            Some(current)
        }
    }
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    let mut max_start = 0;
    let mut max_length = 0;
    for start in 1..1000000 {
        let length = CollatzSequence::iter(start).count();
        if length > max_length {
            max_start = start;
            max_length = length;
        }
    }

    max_start
}
