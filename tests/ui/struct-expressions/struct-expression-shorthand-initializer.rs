//@ run-pass
// Struct Expressions: b25dfa42-3096-48c8-bd1e-274facf0a535
// Struct Expressions: bff40a5f-511f-4c3d-9455-74e279feed66
use std::any::type_name_of_val;

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

fn main() {
    let name = "maria".to_string();
    let age = 50;
    let salary = 50_000;

    let _maria = Employee {
        name: name.clone(),
        age,
        salary,
    };

    let name_type = type_name_of_val(&name);
    let age_type = type_name_of_val(&age);
    let salary_type = type_name_of_val(&salary);

    assert_eq!(name_type, "alloc::string::String");
    assert_eq!(age_type, "u16");
    assert_eq!(salary_type, "u32");
}