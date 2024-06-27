//@ run-pass
// Union Types: 3e3ac7d8-8bac-427d-8898-6ae9fc6277d4
#![allow(dead_code)]

use std::mem;

#[repr(C)]
union CUnion {
    x: u8,
    y: u32,
    z: u64,
}


#[repr(align(16))]
union U16 {
    x: u8,
    y: u32,
}


#[repr(align(32))]
union U32 {
    x: u8,
    y: u32,
}


#[repr(align(64))]
union U64 {
    x: u8,
    y: u32,
}


fn main() {

    assert_eq!(mem::align_of::<CUnion>(), 8); // largest field
    assert_eq!(mem::size_of::<CUnion>(), 8);
    
    assert_eq!(mem::align_of::<U16>(), 16);
    assert_eq!(mem::size_of::<U16>(), 16);

    assert_eq!(mem::align_of::<U32>(), 32);
    assert_eq!(mem::size_of::<U32>(), 32);
        
    assert_eq!(mem::align_of::<U64>(), 64);
    assert_eq!(mem::size_of::<U64>(), 64);
}