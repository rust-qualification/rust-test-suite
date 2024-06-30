//@ run-pass
// Indexed initializer: b28816f0-4bff-4d4c-9d91-2325ec1c6c64

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
}