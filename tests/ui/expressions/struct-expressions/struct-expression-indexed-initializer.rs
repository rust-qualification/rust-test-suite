//@ run-pass
// 2.3.3. Indexed initializer: b28816f0-4bff-4d4c-9d91-2325ec1c6c64
// 2.3.3.2. 886d3a44-fcba-466d-ab0d-ab8bdc418555
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

    assert_eq!(employee.0, "Maria".to_string());
    assert_eq!(employee.1, 50);
    assert_eq!(employee.2, 50000);

    let name_type = type_name_of_val(&name);
    let age_type = type_name_of_val(&age);
    let salary_type = type_name_of_val(&salary);

    assert_eq!(name_type, "alloc::string::String");
    assert_eq!(age_type, "i32");
    assert_eq!(salary_type, "i32");
}