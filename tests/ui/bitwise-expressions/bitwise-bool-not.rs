//@ run-pass
// Bitwise expressions: 4be64039-f32e-4940-bd34-05226e987c3b
use std::ops::Not;

pub fn main() {
    let b: bool = false;
    let c: bool = true;
    
    let not_b = b.not();
    let not_c = c.not();
    
    assert_eq!(not_b, !b);
    assert_eq!(not_c, !c);

    assert_eq!(not_b, true);
    assert_eq!(not_c, false);
}