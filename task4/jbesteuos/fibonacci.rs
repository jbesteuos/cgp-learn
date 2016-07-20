use std::iter::Iterator;

fn main() {

    for i in Fibo::new().take(20) {
        println!("{}", i);
    }
}

struct Fibo {
    first: i32,
    sec: i32,
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo { first: 0, sec: 1 }
    }
}

impl Iterator for Fibo {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let dx = self.first + self.sec;
        self.sec = self.first;
        self.first = dx;
        Some(dx)
    }
}
