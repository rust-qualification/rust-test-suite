//@ run-pass
// Union Types: 26ad2e4a-ff73-4eb4-b16f-d33a6e5d7e7f

#[allow(dead_code)]
fn main() {
    union CopyUnion {
        x: u32,
        y: f64,
    }

    let u = CopyUnion { x: 5 };
    unsafe {
        assert_eq!(u.x, 5);
    }
}