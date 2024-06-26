//@ run-pass
// Struct Expressions: 367c244b-0ac1-42d0-be03-3ad108ee89a3
// Struct Expressions: 123a48b7-d1ea-4efb-a53b-1f89cb837195
// Struct Expressions: 787ef148-019a-4599-b0f2-e155512cab99

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let base_named_union = MyUnion { x: 5 };

    let x: i32 = 5;
    let shorthand_union = MyUnion { x };
}