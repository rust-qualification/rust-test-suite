//@ run-pass
// Comparison expressions: 02e86ee3-3ff4-4756-b7fb-d82534a35556
use core::cmp::PartialOrd;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_less_than_or_equals_b = a.lt(&b);
    let a_less_than_or_equals_c = a.lt(&c);
    
    assert_eq!(a_less_than_or_equals_b, a <= b);
    assert_eq!(a_less_than_or_equals_c, a <= c);

    assert_eq!(a_less_than_or_equals_b, false);
    assert_eq!(a_less_than_or_equals_c, true);
}