// Struct Expressions: 123a48b7-d1ea-4efb-a53b-1f89cb837195

union MyUnion {
    x: i32,
    y: f64,
}

fn main() {
    let named_union_two_fields = MyUnion { x: 5, y: 1.2 };
    //~^ ERROR union expressions should have exactly one field
}