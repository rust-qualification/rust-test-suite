//@ run-pass
// Field Access Expressions: df6f5dfe-b481-40d8-a24b-e69ddd8e94c8

fn main() {
    union MyUnion {
        x: u8,
    }

    let u = MyUnion { x: 5 };
    let x = unsafe { u.x };
    assert_eq!(x, 5);
}