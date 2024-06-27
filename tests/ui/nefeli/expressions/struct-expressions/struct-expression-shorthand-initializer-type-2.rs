// Struct Expressions: bff40a5f-511f-4c3d-9455-74e279feed66

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

fn main() {
    let name = "maria".to_string();
    let age = "maria".to_string();
    let salary = 50_000;

    let _maria = Employee {
        name: name.clone(),
        age,
        salary,
    }; //~^ ERROR mismatched types
}