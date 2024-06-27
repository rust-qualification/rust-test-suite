// Struct Expressions: 56373834-34cc-41e2-a1f7-71528396212e

#[allow(dead_code)]
struct Employee {
    name: String,
    age: u16,
    salary: u32,
}

#[allow(dead_code)]
struct EmployeeNot {
    field: bool,
}

fn main() {
    let maria = EmployeeNot {
        field: true,
    };

    let miguel = Employee {
        name: "miguel".to_string(),
        ..maria
    };
    //~^ ERROR mismatched types
}