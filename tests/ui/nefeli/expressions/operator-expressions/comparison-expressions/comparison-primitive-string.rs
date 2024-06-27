//@ run-pass
// Comparison expressions: c0bf0f1e-01cf-4657-9261-fd2dd4fef6db

pub fn main() {
    let h: String = String::from("aaaa");
    let i: String = String::from("aaaz");

    assert!(h < i);
}
