//@ run-pass
// Bitwise expressions: 8ef4930a-5429-45cc-bbe1-1e651b203d39
use std::ops::Shr;

pub fn main() {
    let num = 12;
    let result = num >> 2;
    assert_eq!(result, 3); 
    assert_eq!(result, Shr::shr(num, 2)); 
}