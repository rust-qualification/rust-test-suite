//@ run-pass
// Struct Expressions: 123a48b7-d1ea-4efb-a53b-1f89cb837195

#[allow(dead_code)]
union MyUnion {
    x: i32,
    y: f64,
}
#[allow(unused_variables)]
fn main() {
    let base_named_union = MyUnion { x: 5 };
}