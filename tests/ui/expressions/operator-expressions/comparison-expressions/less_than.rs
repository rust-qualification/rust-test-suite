//@ run-pass
// Comparison expressions: 4e4cbbd1-2bb4-40e4-b8ad-394526322e95
use std::cmp::PartialOrd;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_less_than_b = a.lt(&b);
    let a_less_than_c = a.lt(&c);
    
    assert_eq!(a_less_than_b, a < b);
    assert_eq!(a_less_than_c, a < c);

    assert_eq!(a_less_than_b, false);
    assert_eq!(a_less_than_c, false);
}