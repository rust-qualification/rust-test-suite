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
    let mut my_union = MyUnion { x: 5 };

    // invalid field access 
    let _ = my_union.1;
    //~^ ERROR no field `1` on type `MyUnion`
    let _ = my_struct.0; 
    //~^ ERROR no field `0` on type `Point`
    let _ = my_union.z;
    //~^ no field `z` on type `MyUnion`
    let _ = my_struct.z; 
    //~^ ERROR no field `z` on type `Point`
}
