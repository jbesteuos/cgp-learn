use std::ops::Mul;
use std::ops::Add;

fn main() {}

pub fn toll<T: Mul + Add + Copy>(x: T, y: T) -> (<T as Mul>::Output, <T as Add>::Output) {

    (x * y, x + y)
}
