//@ run-pass
// Union Types: 218f449a-7973-4157-8a92-87645b9ceedc

#[allow(dead_code)]
fn main() {
    union TupleUnion {
        x: (u32, f64),
        y: (i32, i64),
    }

    let u = TupleUnion { x: (1, 1.5) };
    unsafe {
        assert_eq!(u.x.0, 1);
        assert_eq!(u.x.1, 1.5);
    }

    let u = TupleUnion { y: (5, 1) };
    unsafe {
        assert_eq!(u.y.0, 5);
        assert_eq!(u.y.1, 1);
    }
}