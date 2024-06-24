//@ run-pass
// Bool Types: ee7018f0-eca6-4ec1-b645-ffb98940f3ac

use std::mem;

pub fn main() {

    let true_bool: bool = true;
    let false_bool: bool = false;

    let true_u8: u8 = unsafe { mem::transmute(true_bool) };
    let false_u8: u8 = unsafe { mem::transmute(false_bool) };

    assert!(true_u8 == 1, "Transmuted true bool is invalid");
    assert!(false_u8 == 0, "Transmuted false bool is invalid");
}
