//@ run-pass
// 1.2.5.
#![allow(dead_code)]

use std::mem;

union DefaultUnion {
    x: u8,
    y: u32,
    z: u64,
}

#[repr(align(32))]
union U {
    x: u8,
    y: u32,
}



fn main() {
    let default_union_size = mem::size_of::<DefaultUnion>();
    let default_union_align = mem::align_of::<DefaultUnion>();

    assert_eq!(align_of::<DefaultUnion>(), default_union_align);
    assert_eq!(size_of::<DefaultUnion>(), default_union_size);
    
    let aligned_union_size = mem::size_of::<U>();
    let aligned_union_align = mem::align_of::<U>();
    
    assert_eq!(align_of::<U>(), aligned_union_size);
    assert_eq!(size_of::<U>(), aligned_union_align);
}