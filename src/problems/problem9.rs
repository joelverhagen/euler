use std::clone::Clone;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

struct PythagoreanTriple {
    a: i32,
    b: i32,
    c: i32,
}

impl PythagoreanTriple {
    fn is_valid(&self) -> bool {
        (self.a * self.a) + (self.b * self.b) == (self.c * self.c)
    }

    fn all_gte(&self, value: i32) -> bool {
        self.a >= value && self.b >= value && self.c >= value
    }

    fn is_one_apart(&self) -> bool {
        self.c - self.b == 1 && self.b - self.a == 1
    }

    fn get_min() -> PythagoreanTriple {
        PythagoreanTriple { a: 3, b: 4, c: 5 }
    }
}

impl Display for PythagoreanTriple {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}^2 + {}^2 = {}^2 ({})", self.a, self.b, self.c, self.is_valid())
    }
}

impl Clone for PythagoreanTriple {
    fn clone(&self) -> PythagoreanTriple {
        PythagoreanTriple { a: self.a, b: self.b, c: self.c }
    }

    fn clone_from(&mut self, source: &PythagoreanTriple) {
        self.a = source.a;
        self.b = source.b;
        self.c = source.c;
    }
}

struct PythagoreanTriples {
    current: PythagoreanTriple,
    max_value: i32,
    next: Option<PythagoreanTriple>,
}

impl Iterator for PythagoreanTriples {
    type Item = PythagoreanTriple;

    fn next(&mut self) -> Option<PythagoreanTriple> {
        let mut next = None;
        while next.is_none() && !self.current.all_gte(self.max_value) {
            if self.current.is_valid() {
                next = Some(self.current.clone());
            }

            if self.current.is_one_apart() {
                let new_c = self.current.c + 1;
                self.current = PythagoreanTriple::get_min();
                self.current.c = new_c;
            } else if self.current.c - self.current.b > 1 {
                self.current.b += 1;
                if self.current.c - self.current.b == 1 {
                    self.next = Some(PythagoreanTriple { a: self.current.a + 1, b: self.current.a + 2, c: self.current.c });
                }
            } else if self.next.is_some() {
                self.current = self.next.as_mut().unwrap().clone();
                self.next = None;
            } else if self.current.b - self.current.c > 1 {
                self.current.c += 1;
            }
        }

        next
    }
}

fn get_pythagorean_triples(max_value: i32) -> PythagoreanTriples {
    PythagoreanTriples { current: PythagoreanTriple::get_min(), max_value: max_value, next: None }
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
    let triple = get_pythagorean_triples(998).skip_while(|t| t.a + t.b + t.c != 1000).nth(0).unwrap();
    triple.a * triple.b * triple.c
}
