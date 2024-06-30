//@ run-pass
// Struct Expressions: faea36c2-0a93-4e1c-acd1-62272765b648
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
}