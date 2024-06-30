//@ run-pass
// Bitwise expressions: cf9bcde7-2f46-43cf-bbe2-fcb93d843c2d

#[allow(unused_variables)]
pub fn main() {
    let m: u8 = 3;
    let n: i8 = 4;

    let m_shift = m << n; 
    let l_shift = m >> n; 
}