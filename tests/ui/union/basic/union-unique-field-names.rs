// 1.2.3.
union InvalidUnion {
    x: u8,
    x: u32, //~ ERROR field `x` is already declared
    z: u64, 
}

fn main() {}