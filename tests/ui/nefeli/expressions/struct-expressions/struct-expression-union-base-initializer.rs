// Struct Expressions: 367c244b-0ac1-42d0-be03-3ad108ee89a3
union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let base_named_union = MyUnion { x: 5 };
    let mut new_union = unsafe { MyUnion { ..base_named_union } }; 
    //~^ ERROR union expressions should have exactly one field
    //~| ERROR functional record update syntax requires a struct
}