//@ run-pass
// Bool Types: 415bc9ad-19fb-4b7f-9423-8656c289a6ab

use std::mem;

pub fn main() {

    assert_eq!(mem::size_of::<bool>(), 1);
    assert_eq!(mem::align_of::<bool>(), 1);
}
