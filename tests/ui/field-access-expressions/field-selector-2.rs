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
    let mut my_union = MyUnion { x: 42 };
    let my_tuple = (3, "hi", 1.2);

    // invalid field access 
    let _ = my_union.1;
    //~^ ERROR no field `1` on type `Point`
    let _ = p.0; 
    //~^ ERROR no field `0` on type `MyUnion`
    let _ = my_tuple.first;
    //~^ ERROR no field `first` on type `({integer}, &str, {float})`
}
