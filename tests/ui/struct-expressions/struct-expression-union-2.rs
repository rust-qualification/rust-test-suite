// Struct Expressions: 367c244b-0ac1-42d0-be03-3ad108ee89a3
// Struct Expressions: 123a48b7-d1ea-4efb-a53b-1f89cb837195
// Struct Expressions: 787ef148-019a-4599-b0f2-e155512cab99

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let base_named_union = MyUnion { x: 5 };
    let mut new_union = unsafe { MyUnion { ..base_named_union } }; 
    //~^ ERROR union expressions should have exactly one field
    //~| ERROR functional record update syntax requires a struct

    let named_union_two_fields = MyUnion { x: 5, y: 1.2 };
    //~^ ERROR union expressions should have exactly one field

    let x: i32 = 5;
    let y: f64 = 5.2;
    let shorthand_union_two_fields = MyUnion { x, y };
    //~^ ERROR union expressions should have exactly one field
}