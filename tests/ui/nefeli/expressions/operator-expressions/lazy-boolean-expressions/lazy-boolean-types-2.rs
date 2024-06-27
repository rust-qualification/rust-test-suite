// Lazy Boolean Opearations: 8ee2a7bd-59c9-4479-9516-5519af52aa3b

pub fn main() {
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