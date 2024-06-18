//@ run-pass
// 1.2.1.
#![allow(dead_code)]
use std::mem;

fn main() {
    union MyUnion {
        x: u8,
        y: u32,
        z: u64,
    }

    let union_size = mem::size_of::<MyUnion>();
    let largest_size = mem::size_of::<u64>();

    assert_eq!(union_size, largest_size);
}
