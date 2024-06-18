// 2.3.8.

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    // 2.3.8.2
    let base_named_union = MyUnion { x: 5 };

    // 2.3.8.3
    let x: i32 = 5;
    let shorthand_union = MyUnion { x };

    // 2.3.8.1
    let mut new_union = unsafe { MyUnion { ..base_named_union } }; // ERROR
    //~^ ERROR union expressions should have exactly one field
    //~| ERROR functional record update syntax requires a struct
}