//@ run-pass
// Lazy Boolean Opearations: 266ee141-7c6b-4738-bc33-24a72e8e3d99
// Lazy Boolean Opearations: 0735bf32-a30d-4a64-8ca1-8ad873bc9e04

pub fn main() {
    assert!(!(false && panic!()));
    assert!(true || panic!());
}