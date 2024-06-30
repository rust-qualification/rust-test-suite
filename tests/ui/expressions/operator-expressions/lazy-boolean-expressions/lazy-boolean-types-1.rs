//@ run-pass
// Lazy Boolean Opearations: 8ee2a7bd-59c9-4479-9516-5519af52aa3b

pub fn main() {
    assert!(true && true);
    assert!(true || false);
}