//@ run-pass
// Struct Expressions: b25dfa42-3096-48c8-bd1e-274facf0a535

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
}