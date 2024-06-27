//@ run-pass
// Struct Expressions: f80fa0d8-b7af-4813-9bcf-c93180afaff8

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

#[allow(unused_variables)]
fn main() {
    let maria = Employee {
        name: "maria".to_string(),
        age: 50,
        salary: 50_000,
    };


    let miguel = Employee {
        name: "miguel".to_string(),
        ..maria
    };
}