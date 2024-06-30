//@ run-pass
// Union Types: e1c2f91e-7a68-48bd-b103-66749e82703c
use std::mem::ManuallyDrop;

#[allow(dead_code)]
union GenericUnion<X, Y>
{
    x: ManuallyDrop<X>,
    y: ManuallyDrop<Y>,
}

fn main() {
    let u: GenericUnion<i32, String> = GenericUnion { x: ManuallyDrop::new(42) };

    unsafe {
        assert_eq!(ManuallyDrop::into_inner(u.x), 42);
    }
}