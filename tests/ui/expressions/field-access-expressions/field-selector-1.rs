//@ run-pass
// Field Access Expressions: 7fc7d0a9-9066-4c99-81a5-37bd9ca6b223
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

    let my_tuple = (3, "hi", 1.2);
    
    let first = my_tuple.0;
    let second = my_tuple.1;
    let third = my_tuple.2;
    
    assert_eq!(first, 3);
    assert_eq!(second, "hi");
    assert_eq!(third, 1.2);
}
