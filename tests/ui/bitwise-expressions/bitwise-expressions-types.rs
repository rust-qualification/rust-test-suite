// Bitwise expressions: 58fa3a4b-f15b-41f3-b8b3-2f607b8b8688

pub fn main() {
    let a: bool = true;
    let b: bool = false;
    
    let a_or_b = a | b;
    let a_and_b = a & b;
    let a_xor_b = a ^ b;
    let not_b = !b;
    let a_shift = a << b; 
    //~^ ERROR no implementation for `bool << bool`
    let b_shift = a >> b; 
    //~^ ERROR no implementation for `bool >> bool`

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

    // invalid types
    let g: f32 = 1.2;
    let h: f32 = 1.5;
    
    let g_or_h = g | h; 
    //~^ ERROR no implementation for `f32 | f32`
    let g_and_h = g & h;
    //~^ ERROR no implementation for `f32 & f32`
    let g_xor_h = g ^ h;
    //~^ ERROR no implementation for `f32 ^ f32`
    let not_h = !h;
    //~^ ERROR cannot apply unary operator `!` to type `f32`
    let g_shift = g << h; 
    //~^ ERROR no implementation for `f32 << f32`
    let h_shift = g >> h; 
    //~^ ERROR no implementation for `f32 >> f32`
    
    // invalid types
    let i: char = 'a';
    let j: char = 'b';
    
    let i_or_j = i | j;
    //~^ ERROR no implementation for `char | char`
    let i_and_j = i & j;
    //~^ ERROR no implementation for `char & char`
    let i_xor_j = i ^ j;
    //~^ ERROR no implementation for `char ^ char`
    let not_j = !j;
    //~^ ERROR cannot apply unary operator `!` to type `char`
    let i_shift = i << j; 
    //~^ ERROR no implementation for `char << char`
    let j_shift = i >> j; 
    //~^ ERROR no implementation for `char >> char`
    

    // invalid types
    let k:  String = String::from("Hello");
    let l:  String = String::from("world");

    let k_or_l = k | l;
    //~^ ERROR no implementation for `String | String`
    let k_and_l = k & l;
    //~^ ERROR no implementation for `String & String`
    let k_xor_l = k ^ l;
    //~^ ERROR no implementation for `String ^ String`
    let not_l = !l;   
    //~^ ERROR cannot apply unary operator `!` to type `String`
    let k_shift = k << l; 
    //~^ ERROR no implementation for `String << String`
    let l_shift = k >> l; 
    //~^ ERROR no implementation for `String >> String`
    

    
    // invalid types
    let m: i8 = 3;
    let n: bool = false;
    
    let m_or_n = m | n;
    //~^ ERROR no implementation for `i8 | bool`
    let m_and_n = m & n;
    //~^ ERROR no implementation for `i8 & bool`
    let m_xor_n = m ^ n;
    //~^ ERROR no implementation for `i8 ^ bool`
    let m_shift = m << n; 
    //~^ ERROR no implementation for `i8 << bool`
    let n_shift = m >> n; 
    //~^ ERROR no implementation for `i8 >> bool`
    
}