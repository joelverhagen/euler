struct Triangles {
    next: u64,
}

impl Triangles {
    fn iter() -> Triangles {
        Triangles { next: 1 }
    }
}

impl Iterator for Triangles {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let output = (1..self.next + 1).fold(0, |s, v| s + v);
        self.next += 1;
        Some(output)
    }
}

struct Factors {
    value: u64,
    next_calculated: u64,
    next_divisor: Option<u64>,
    sqrt: u64,
}

impl Factors {
    fn iter(value: u64) -> Factors {
        let sqrt = (value as f64).sqrt() as u64;
        Factors { value: value, next_calculated: 1, next_divisor: None, sqrt: sqrt }
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut output = None;

        if self.next_divisor.is_some() {
            output = self.next_divisor;
            self.next_divisor = None;
        }

        while output.is_none() && self.next_calculated <= self.sqrt {
            if self.value % self.next_calculated == 0 {
                output = Some(self.next_calculated);

                let next_divisor = self.value / self.next_calculated;
                if (next_divisor == self.value || next_divisor != self.sqrt) && self.value != 1 {
                    self.next_divisor = Some(next_divisor);
                }                
            }

            self.next_calculated += 1;
        }

        output
    }
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    Triangles::iter().skip_while(|&t| Factors::iter(t).count() < 500).nth(0).unwrap()
}
