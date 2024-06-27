//@ run-pass
// Comparison expressions: 35bd3b3e-959c-41d9-936d-e70c110dd8b5

pub fn main() {
    let x = 1;
    let y = 2;

    assert!(x != y);

    // can still use them
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}
