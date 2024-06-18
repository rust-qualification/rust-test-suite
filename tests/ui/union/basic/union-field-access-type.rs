//@ run-pass
// 2.2.1.
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    union MyUnion {
        x: u8,
        y: u32,
    }

    unsafe {
        let mut u = MyUnion { x: 5 };
        let x = u.x;
        assert!(type_of(&x) == type_of(&5u8));
        u.y = 10;
        let y = u.y;
        assert!(type_of(&y) == type_of(&10u32));
    }
}
