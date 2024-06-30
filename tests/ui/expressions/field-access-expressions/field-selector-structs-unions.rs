//@ run-pass
// Field Access Expressions: 6e237bb8-5e9d-4ea7-ad47-798f21044638

struct Point {
    x: i32,
    y: i32,
}

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let my_struct = Point { x: 1, y: 2 };
    let x = my_struct.x;
    let y = my_struct.y;
    assert_eq!(x, 1);
    assert_eq!(y, 2);

    let mut my_union = MyUnion { x: 5 };
    my_union.y = 1.2;
}