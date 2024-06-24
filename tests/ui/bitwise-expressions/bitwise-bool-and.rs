//@ run-pass
// Bitwise expressions: ba09332a-f44d-46e7-9dd2-38885daa3435
use std::ops::BitAnd;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_and_b = a.bitand(b);
    let a_and_c = a.bitand(c);
    
    assert_eq!(a_and_b, a & b);
    assert_eq!(a_and_c, a & c);

    assert_eq!(a_and_b, false);
    assert_eq!(a_and_c, true);
}