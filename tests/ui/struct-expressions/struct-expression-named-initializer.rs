//@ run-pass
// Struct Expressions: faea36c2-0a93-4e1c-acd1-62272765b648
// Struct Expressions: f3930897-1745-4c56-9481-be35348298d5
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

    let name_type = type_name_of_val(&maria.name);
    let age_type = type_name_of_val(&maria.age);
    let salary_type = type_name_of_val(&maria.salary);

    assert_eq!(name_type, "alloc::string::String");
    assert_eq!(age_type, "u16");
    assert_eq!(salary_type, "u32");
}