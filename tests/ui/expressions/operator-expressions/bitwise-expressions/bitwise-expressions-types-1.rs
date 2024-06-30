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
    
    let e: i16 = 3;
    let f: i16 = 4;
    
    let e_or_f = e | f;
    let e_and_f = e & f;
    let e_xor_f = e ^ f;
    let not_f = !f;
    let e_shift = e << f; 
    let f_shift = e >> f; 

    let g: i64 = 3;
    let h: i64 = 4;
    
    let g_or_h = g | h;
    let g_and_h = g & h;
    let g_xor_h = g ^ h;
    let not_h = !h;
    let g_shift = g << h; 
    let h_shift = g >> h; 


    
    let i: isize = 3;
    let j: isize = 4;
    
    let i_or_j = i | j;
    let i_and_j = i & j;
    let i_xor_j = i ^ j;
    let not_j = !j;
    let i_shift = i << j; 
    let j_shift = i >> j; 

    let k: u8 = 3;
    let l: u8 = 4;
    
    let k_or_l = k | l;
    let k_and_l = k & l;
    let k_xor_l = k ^ l;
    let not_l = !l;
    let k_shift = k << l; 
    let l_shift = k >> l; 
}