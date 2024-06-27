//@ run-pass
// Struct Expressions: 56373834-34cc-41e2-a1f7-71528396212e
use std::any::type_name_of_val;

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

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

    let maria_type = type_name_of_val(&maria);
    let miguel_type = type_name_of_val(&miguel);

    assert_eq!(maria_type, miguel_type)
}