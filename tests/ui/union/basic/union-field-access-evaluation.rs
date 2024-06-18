//@ run-pass
// 2.2.5.

fn main() {
    union MyUnion {
        x: u8,
    }

    let u = MyUnion { x: 5 };
    let x = unsafe { u.x };
    assert_eq!(x, 5);
}