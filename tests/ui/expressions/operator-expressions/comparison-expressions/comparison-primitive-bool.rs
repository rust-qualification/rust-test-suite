//@ run-pass
// Comparison expressions: ce008a05-6b67-4e02-85c6-38374ab3a561

pub fn main() {
    let c = true;
    let d = false;

    assert!(d < c);
}
