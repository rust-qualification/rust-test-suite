// Bitwise expressions: 63e0629e-2e63-405d-bd20-a2feebb64921

pub fn main() {
    let m: u8 = 3;
    let n: i8 = 4;
    
    // invalid types
    let m_or_n = m | n;
    //~^ ERROR mismatched types
    let m_and_n = m & n;
    //~^ ERROR mismatched types
    let m_xor_n = m ^ n;
    //~^ ERROR mismatched types
}