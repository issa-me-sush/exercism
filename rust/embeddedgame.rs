#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
(dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
struct EvenIterator<I: Iterator<Item = T>, T> {
iter: I,
}impl<I: Iterator<Item = T>, T> Iterator for EvenIterator<I, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|x| {
            self.iter.next();
            Some(x)
        })
    }
}

EvenIterator { iter }
}

pub struct Position(pub i16, pub i16);

impl Position {
pub fn manhattan(&self) -> i16 {
self.0.abs() + self.1.abs()
}
}