// Struct Expressions: 787ef148-019a-4599-b0f2-e155512cab99

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let x: i32 = 5;
    let y: f64 = 5.2;
    let shorthand_union_two_fields = MyUnion { x, y };
    //~^ ERROR union expressions should have exactly one field
}