// Union types: 6356434c-6cdb-47cd-8e99-cff31c5bef14
union InvalidUnion {
    x: u8,
    x: u32, //~ ERROR field `x` is already declared
    z: u64, 
}

fn main() {}