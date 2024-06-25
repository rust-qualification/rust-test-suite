//@ run-pass
// Comparison expressions: 17b4e671-32b1-4dea-bd1c-8692e6bc6eb9
// Comparison expressions: c0bf0f1e-01cf-4657-9261-fd2dd4fef6db
// Comparison expressions: 6a2df541-702c-45c2-9c93-e01610846161
// Comparison expressions: ce008a05-6b67-4e02-85c6-38374ab3a561

pub fn main() {
    let x: i32 = 1;
    let y: i32 = 2;

    assert!(x < y);

    let a = 'a';
    let b = 'b';

    assert!(a < b);

    let c = true;
    let d = false;

    assert!(d < c);

    let h: String = String::from("aaaa");
    let i: String = String::from("aaaz");

    assert!(h < i);

    let j = 1.4;
    let k = 2.2;

    assert!(j < k);
}
