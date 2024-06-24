//@ run-pass
// Bitwise expressions: e2dcd73e-8e58-4302-ad00-a0443924c533
use std::ops::Shl;

pub fn main() {
    let num = MyNumber(3);
    let result = num << 2;
    assert_eq!(result.0, 12); 
    assert_eq!(result.0, std::ops::Shl::shl(num, 2)); 
}