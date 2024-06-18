//@ run-pass
// 2.2.2.
#[repr(C)]
union MyUnion {
    x: i32,
    y: u32,
}

fn main() {
    let u = MyUnion { x: 2 };

    unsafe {
        assert_eq!(u.x, 2);
    }
}