// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    //TODO: ugly code
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    return (quotient, remainder);
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    std::iter::empty()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        unimplemented!("implement `fn manhattan`")
    }
}