// Bitwise expressions: d7a0044c-a6c7-4a18-8b26-d6f3ff619e7b

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    
    // invalid types
    let a_shift = a << b; 
    //~^ ERROR no implementation for `bool << bool`
    let b_shift = a >> b; 
    //~^ ERROR no implementation for `bool >> bool`
}