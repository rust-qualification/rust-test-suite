//@ run-pass
// 1.1.3.

use std::mem;

pub fn main() {

    let true_bool: bool = true;
    let false_bool: bool = false;

    let true_u8: u8 = unsafe { mem::transmute(true_bool) };
    let false_u8: u8 = unsafe { mem::transmute(false_bool) };

    assert!(true_u8 == 1, "Transmuted true bool is invalid");
    assert!(false_u8 == 0, "Transmuted false bool is invalid");
}
