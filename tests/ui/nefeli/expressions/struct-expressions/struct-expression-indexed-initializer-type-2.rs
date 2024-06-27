// Indexed initializer:  886d3a44-fcba-466d-ab0d-ab8bdc418555
struct Employee(String, i32, i32);

fn main() {
    let employee = Employee(
        "Maria".to_string(),   
        "Maria".to_string(),                       
        50000                     
    ); //~^ ERROR mismatched types
}