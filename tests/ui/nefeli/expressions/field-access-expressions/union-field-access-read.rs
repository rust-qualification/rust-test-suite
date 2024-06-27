//@ run-pass
// Field Access Expressions: 643ad9f7-3e86-48ea-8493-a6741596206f
// Field Access Expressions: e35147ab-017e-454f-b748-6b78f8c5063b
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