//@ run-pass
// Comparison expressions: 04aa0dda-0706-44cb-9ab3-7945aeb58bb3
use core::cmp::PartialEq;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_equals_b = a.eq(&b);
    let a_equals_c = a.eq(&c);
    
    assert_eq!(a_equals_b, a == b);
    assert_eq!(a_equals_c, a == c);

    assert_eq!(a_equals_b, false);
    assert_eq!(a_equals_c, true);
}