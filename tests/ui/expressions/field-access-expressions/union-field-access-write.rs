//@ run-pass
// Field Access Expressions: 8512464b-4793-4901-8769-d674cedf1a69

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