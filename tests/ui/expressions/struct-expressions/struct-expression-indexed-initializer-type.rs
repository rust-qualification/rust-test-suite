//@ run-pass
// Indexed initializer:  886d3a44-fcba-466d-ab0d-ab8bdc418555
use std::any::type_name_of_val;

struct Employee(String, i32, i32);

fn main() {
    let employee = Employee(
        "Maria".to_string(),   
        50,                       
        50000                     
    );

    let name = "Maria".to_string();
    let age = 50;
    let salary = 50000;

    assert_eq!(employee.0, name);
    assert_eq!(employee.1, age);
    assert_eq!(employee.2, salary);

    let name_type = type_name_of_val(&name);
    let age_type = type_name_of_val(&age);
    let salary_type = type_name_of_val(&salary);

    assert_eq!(name_type, "alloc::string::String");
    assert_eq!(age_type, "i32");
    assert_eq!(salary_type, "i32");
}