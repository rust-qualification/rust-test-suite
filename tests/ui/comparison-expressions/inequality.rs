//@ run-pass
// Comparison expressions: ac7610ba-251c-4854-bffc-3c0b9f8fdb7c
use core::cmp::PartialEq;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_not_equals_b = a.ne(&b);
    let a_not_equals_c = a.ne(&c);
    
    assert_eq!(a_not_equals_b, a != b);
    assert_eq!(a_not_equals_c, a != c);

    assert_eq!(a_not_equals_b, true);
    assert_eq!(a_not_equals_c, false);
}