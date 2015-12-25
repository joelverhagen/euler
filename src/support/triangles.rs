pub struct Triangles {
    n: u64,
    value: u64,
}

impl Triangles {
    pub fn iter() -> Triangles {
        Triangles { n: 1, value: 0 }
    }
}

impl Iterator for Triangles {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.value;
        self.value += self.n;
        self.n += 1;
        Some(value)
    }
}
