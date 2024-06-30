// Struct Expressions: f3930897-1745-4c56-9481-be35348298d5

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

fn main() {
    let maria = Employee {
        name: "maria".to_string(),
        age: "maria".to_string(), //~ ERROR mismatched types
        salary: 50_000,
    }; 
}