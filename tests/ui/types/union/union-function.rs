// Union Types
fn main() {
    union MyUnion {
        field: u8,
    
        fn func() {}
        //~^ ERROR functions are not allowed in union definitions
        //~| HELP unlike in C++, Java, and C#, functions are declared in `impl` blocks
        //~| HELP see https://doc.rust-lang.org/book/ch05-03-method-syntax.html for more information
    }
}
