//@ run-pass
// Union Types: 93059842-a3be-4dd1-92c7-1b79f40e252f
#[allow(dead_code)]
union GenericUnion<X, Y>
where
    X: Copy,
    Y: Copy,
{
    x: X,
    y: Y,
}

fn main() {
    let u: GenericUnion<i32, ()> = GenericUnion { x: 2 };

    unsafe {
        assert_eq!(u.x, 2);
    }
}