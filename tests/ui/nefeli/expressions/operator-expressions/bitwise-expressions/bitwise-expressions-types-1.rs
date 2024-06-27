//@ run-pass
// Bitwise expressions: 58fa3a4b-f15b-41f3-b8b3-2f607b8b8688
#[allow(unused_variables)]
pub fn main() {
    let a: bool = true;
    let b: bool = false;
    
    let a_or_b = a | b;
    let a_and_b = a & b;
    let a_xor_b = a ^ b;
    let not_b = !b;

    let c: i8 = 0b1010;
    let d: i8 = 0b0101;
    
    let c_or_d = c | d;
    let c_and_d = c & d;
    let c_xor_d = c ^ d;
    let not_d = !d;
    let c_shift = c << d; 
    let d_shift = c >> d; 
    
    let e: i8 = 3;
    let f: i8 = 4;
    
    let e_or_f = e | f;
    let e_and_f = e & f;
    let e_xor_f = e ^ f;
    let not_f = !f;
    let e_shift = e << f; 
    let f_shift = e >> f; 
}