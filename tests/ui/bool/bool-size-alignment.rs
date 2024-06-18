//@ run-pass
// 1.1.2.

use std::mem;

pub fn main() {

    assert_eq!(mem::size_of::<bool>(), 1, "Size of bool is not 1 byte");
    assert_eq!(mem::align_of::<bool>(), 1, "Alignment of bool is not 1 byte");
}
