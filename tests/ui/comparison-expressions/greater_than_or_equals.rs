//@ run-pass
// Comparison expressions: 8c9f42c4-79b2-4017-ace4-81d08549c196
use std::cmp::PartialOrd;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_greater_than_or_equals_b = a.ge(&b);
    let a_greater_than_or_equals_c = a.ge(&c);
    
    assert_eq!(a_greater_than_or_equals_b, a >= b);
    assert_eq!(a_greater_than_or_equals_c, a >= c);

    assert_eq!(a_greater_than_or_equals_b, true);
    assert_eq!(a_greater_than_or_equals_c, true);
}