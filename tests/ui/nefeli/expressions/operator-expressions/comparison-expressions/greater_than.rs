//@ run-pass
// Comparison expressions: 9289fb9d-67b5-4f30-bf22-2b9d646677aa
use std::cmp::PartialOrd;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_greater_than_b = a.gt(&b);
    let a_greater_than_c = a.gt(&c);
    
    assert_eq!(a_greater_than_b, a > b);
    assert_eq!(a_greater_than_c, a > c);

    assert_eq!(a_greater_than_b, true);
    assert_eq!(a_greater_than_c, false);
}