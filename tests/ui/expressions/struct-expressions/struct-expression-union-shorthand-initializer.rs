//@ run-pass
// Struct Expressions: 787ef148-019a-4599-b0f2-e155512cab99

#[allow(dead_code)]
union MyUnion {
    x: i32,
    y: f64,
}
#[allow(unused_variables)]
fn main() {
    let x: i32 = 5;
    let shorthand_union = MyUnion { x };
}