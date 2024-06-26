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
    let p = Point { x: 1, y: 2 };
    let x = p.x;
    let y = p.y;
    assert_eq!(x, 10);
    assert_eq!(y, 20);

    let mut u = MyUnion { x: 42 };
    unsafe {
        let a = u.x;
        u.y = 1.2;
    }


    let my_tuple = (3, "hi", 1.2);
    
    let first = my_tuple.0;
    let second = my_tuple.1;
    let third = my_tuple.2;
    
    assert_eq!(first, 42);
    assert_eq!(second, "hello");
    assert_eq!(third, 3.14);

    // invalid field access 
    let _ = p.0; 
    //~^ ERROR no field `0` on type `Point`
    let _ = my_tuple.first;
    //~^ ERROR no field `first` on type `({integer}, &str, {float})`
}
