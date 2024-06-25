// Lazy Boolean Opearations: 8ee2a7bd-59c9-4479-9516-5519af52aa3b
// Lazy Boolean Opearations: 44418fad-561e-488e-9a04-30a4701aa735

use std::any::type_name;
use std::any::type_name_of_val;

pub fn main() {
    let result: bool = true && true;
    assert_eq!(type_name::<bool>(), type_name_of_val(&result));

    assert!(true && false);
    assert!(true || false);
    assert!(1 && 1); 
    //~^ ERROR mismatched types
    //~| ERROR mismatched types
    assert!(1.5 && 1.5); 
    //~^ ERROR mismatched types
    //~| ERROR mismatched types
    assert!(1.5 && 1); 
    //~^ ERROR mismatched types
    //~| ERROR mismatched types    
    assert!(false && 1); 
    //~^ ERROR mismatched types
    assert!('a' && 'b'); 
    //~^ ERROR mismatched types
    //~| ERROR mismatched types    
    assert!('a' || true); 
    //~^ ERROR mismatched types
    assert!("alpha" && "beta");
    //~^ ERROR mismatched types
    //~| ERROR mismatched types     
    assert!("alpha" || true); 
    //~^ ERROR mismatched types
}