//@ run-pass
// Bool Types: 4980f9e0-3291-41ac-bef8-21acdf3545ba
use std::any::type_name;
use std::any::type_name_of_val;

pub fn main() {
    let true_literal = true;
    let false_literal = false;
    assert_eq!(type_name::<bool>(), type_name_of_val(&true_literal));
    assert_eq!(type_name::<bool>(), type_name_of_val(&false_literal));
}