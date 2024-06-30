// Union Types: 218f449a-7973-4157-8a92-87645b9ceedc

fn main() {
    union InvalidTupleUnion {
        x: (String, f64), //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
}