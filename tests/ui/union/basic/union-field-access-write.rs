//@ run-pass
// 2.2.3.

#[repr(C)]
union MyUnion {
    x: i32,
    y: u32,
}

fn main() {
    let mut u = MyUnion { x: 2 };
    u.x = 3;
    unsafe {
        assert_eq!(u.x, 3);
    }
}