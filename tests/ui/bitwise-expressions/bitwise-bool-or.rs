//@ run-pass
// Bitwise expressions: 071612e5-2ff0-4fe2-827b-a2eb4dbd3aaf
use std::ops::BitOr;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_or_b = a.bitor(b);
    let a_or_c = a.bitor(c);
    
    assert_eq!(a_or_b, a | b);
    assert_eq!(a_or_c, a | c);

    assert_eq!(a_or_b, true);
    assert_eq!(a_or_c, true);
}