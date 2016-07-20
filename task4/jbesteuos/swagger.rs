use std::fmt::Display;
use std::fmt;
fn main() {}

struct Swagger<T: Display> {
    olaf: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#swag {} #yolo", self.olaf)
    }
}
