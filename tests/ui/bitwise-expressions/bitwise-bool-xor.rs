//@ run-pass
// Bitwise expressions: e74c08ba-07fe-4c10-b68e-1028b289ea77
use std::ops::BitXor;

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    let c: bool = true;
    
    let a_xor_b = a.bitxor(b);
    let a_xor_c = a.bitxor(c);
    
    assert_eq!(a_xor_b, a ^ b);
    assert_eq!(a_xor_c, a ^ c);

    assert_eq!(a_xor_b, true);
    assert_eq!(a_xor_c, false);
}